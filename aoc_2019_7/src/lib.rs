use aoc_2019_2::{ComputationResult, IntComputer};
use core::mem;
use permutohedron::LexicalPermutation;
use std::borrow::BorrowMut;
use std::io::{BufRead, BufReader};
use std::process::Output;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

pub fn find_largest_permutation(memory: Vec<i32>, input: Vec<i32>) -> Result<i32, String> {
    let memory = memory;
    let mut input: Vec<i32> = input;
    let mut results = Vec::new();

    loop {
        results.push(compute_result_for_sequence(&memory, &input)?);

        if !input.next_permutation() {
            break;
        }
    }

    results
        .into_iter()
        .max()
        .ok_or("No valid value found.".to_string())
}

fn compute_result_for_sequence(memory: &Vec<i32>, sequence: &Vec<i32>) -> Result<i32, String> {
    let mut result = 0;

    for &first_input in sequence {
        result = compute_result(memory.clone(), first_input, result)?
    }

    Ok(result)
}

pub fn find_largest_permutation_for_feedback_sequence(
    memory: Vec<i32>,
    input: Vec<i32>,
) -> Result<i32, String> {
    let memory = memory;
    let mut input: Vec<i32> = input;
    let mut results = Vec::new();

    loop {
        results.push(compute_result_for_feedback_sequence(&memory, &input)?);

        if !input.next_permutation() {
            break;
        }
    }

    results
        .into_iter()
        .max()
        .ok_or("No valid value found.".to_string())
}

fn compute_result_for_feedback_sequence(
    memory: &Vec<i32>,
    sequence: &Vec<i32>,
) -> Result<i32, String> {
    let (first_input_sender, first_input_receiver) = mpsc::channel();
    let mut input_receiver = first_input_receiver;

    for idx in 1..sequence.len() {
        let sequence_number = sequence[idx];

        let (output_sender, output_receiver) = mpsc::channel();
        AsyncIntComputer::new(memory.clone(), input_receiver, output_sender);

        let (next_input_sender, next_input_receiver) = mpsc::channel();

        input_receiver = next_input_receiver;

        let writer = ChannelWriter::new(next_input_sender);

        writer.write_int(sequence_number);

        OutputForwarder::new(output_receiver, writer);
    }

    let (output_sender, output_receiver) = mpsc::channel();
    AsyncIntComputer::new(memory.clone(), input_receiver, output_sender);

    let writer = ChannelWriter::new(first_input_sender);
    writer.write_int(sequence[0]);
    writer.write_int(0);

    let final_forwarder = OutputForwarder::new(output_receiver, writer);

    final_forwarder
        .join()
        .map_err(|err| format!("Failed to join final forwarder."))?
        .ok_or(format!("Could not produce output."))
}

fn compute_result(memory: Vec<i32>, first_input: i32, second_input: i32) -> Result<i32, String> {
    let input = format!("{}\n{}\n", first_input, second_input);
    let mut output: Vec<u8> = Vec::new();
    let mut computer = IntComputer::new(memory, input.as_bytes(), &mut output);
    assert_eq!(computer.compute(), ComputationResult::Success);
    let mut x: Vec<Result<i32, String>> = String::from_utf8(output)
        .unwrap()
        .split("\n")
        .filter(|line| line.contains("Output value:"))
        .flat_map(|line: &str| {
            line.find(":").map(|idx| {
                line[idx + 1..]
                    .trim()
                    .parse()
                    .map_err(|err| format!("Could not parse integer: {}", err))
            })
        })
        .collect();

    let result = mem::replace(&mut x[0], Ok(42));

    result
}

pub struct AsyncIntComputer {
    handle: JoinHandle<()>,
}

impl AsyncIntComputer {
    pub fn new<'a>(
        memory: Vec<i32>,
        input: mpsc::Receiver<u8>,
        output: mpsc::Sender<u8>,
    ) -> AsyncIntComputer {
        let handle = thread::spawn(move || {
            let mut channel_writer = ChannelWriter::new(output);
            let mut computer = aoc_2019_2::IntComputer::new(
                memory,
                BufReader::new(ChannelReader::new(input)),
                &mut channel_writer,
            );
            computer.compute();
        });
        AsyncIntComputer { handle }
    }

    pub fn join(self) -> thread::Result<()> {
        self.handle.join()
    }
}

pub struct ChannelWriter {
    tx: mpsc::Sender<u8>,
}

impl ChannelWriter {
    pub fn new(tx: mpsc::Sender<u8>) -> ChannelWriter {
        ChannelWriter { tx }
    }

    pub fn write_int(&self, value: i32) -> Result<(), SendError<i32>> {
        let output: Vec<u8> = format!("{}\n", value).into();

        for byte in output {
            self.tx.send(byte).map_err(|err| SendError(value))?
        }

        Ok(())
    }
}

impl io::Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        for idx in 0..buf.len() {
            if let Err(err) = self.tx.send(buf[idx]) {
                return Ok(idx);
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

pub struct ChannelReader {
    rx: mpsc::Receiver<u8>,
}

impl ChannelReader {
    pub fn new(rx: mpsc::Receiver<u8>) -> ChannelReader {
        ChannelReader { rx }
    }
}

impl io::Read for ChannelReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let len = buf.len();

        if len == 0 {
            return Ok(0);
        }

        match self.rx.recv() {
            Ok(next_byte) => buf[0] = next_byte,
            Err(err) => return Ok(0),
        };

        for idx in 1..len {
            match self.rx.try_recv() {
                Ok(next_byte) => buf[idx] = next_byte,
                Err(err) => return Ok(idx),
            }
        }

        Ok(len)
    }
}

pub struct OutputForwarder {
    handle: JoinHandle<Option<i32>>,
}

impl OutputForwarder {
    pub fn new(input: mpsc::Receiver<u8>, writer: ChannelWriter) -> OutputForwarder {
        let handle = thread::spawn(move || {
            let reader = BufReader::new(ChannelReader::new(input));
            let mut last_forwarded_value = None;

            for line in reader.lines() {
                let line = line.unwrap();
                let output_pattern = "Output value:";

                if let Some(idx) = line.find(output_pattern) {
                    let parsed_output = line[idx + output_pattern.len()..].trim().parse().unwrap();

                    last_forwarded_value = Some(parsed_output);

                    writer.write_int(parsed_output);
                }
            }

            last_forwarded_value
        });

        OutputForwarder { handle }
    }

    pub fn join(self) -> thread::Result<Option<i32>> {
        self.handle.join()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program_one() {
        let memory = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        assert_eq!(
            compute_result_for_sequence(&memory, &vec![4, 3, 2, 1, 0]),
            Ok(43210)
        );
    }

    #[test]
    fn test_feedback_loop_program_one() {
        let memory = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let sequence = vec![9, 8, 7, 6, 5];

        assert_eq!(
            compute_result_for_feedback_sequence(&memory, &sequence),
            Ok(139629729)
        );
    }
}
