use std::fs;
use std::io;
use std::io::BufRead;

use crate::Command::*;
use crate::ComputationResult::{Failure, Success};
use std::ops::Index;
use std::str::FromStr;

pub struct IntComputer<'a, I: io::BufRead, O: io::Write> {
    memory: Memory,
    pointer: isize,
    relative_base: usize,
    input: I,
    output: &'a mut O,
}

pub struct Memory {
    memory: Vec<i64>,
}

impl Memory {
    fn new(memory: Vec<i64>) -> Memory {
        Memory { memory }
    }

    pub fn get(&self, idx: usize) -> i64 {
        self.verify_index(idx);
        self.memory.get(idx).map(|&v| v).unwrap_or(0)
    }

    fn put(&mut self, idx: usize, value: i64) {
        self.verify_index(idx);

        if idx >= self.memory.len() {
            self.memory.resize(idx + 1, 0)
        }

        self.memory[idx] = value;
    }

    fn verify_index(&self, idx: usize) {
        if idx < 0 {
            panic!("Index {} cannot below than 0.", idx);
        }
    }

    pub fn memory(&self) -> &Vec<i64> {
        &self.memory
    }
}

impl<Idx> std::ops::Index<Idx> for Memory
where
    Idx: std::slice::SliceIndex<[i64], Output = [i64]>,
{
    type Output = [i64];

    fn index(&self, index: Idx) -> &Self::Output {
        &self.memory[index]
    }
}

#[derive(PartialEq, Debug)]
pub enum ComputationResult {
    Success,
    Failure { error: String },
}

enum Command {
    Add(InputParameter, InputParameter, OutputParameter),
    Multiply(InputParameter, InputParameter, OutputParameter),
    Input(OutputParameter),
    Output(InputParameter),
    JumpIfTrue(InputParameter, InputParameter),
    JumpIfFalse(InputParameter, InputParameter),
    LessThan(InputParameter, InputParameter, OutputParameter),
    Equals(InputParameter, InputParameter, OutputParameter),
    AdjustRelativeBase(InputParameter),
    Stop,
}

#[derive(Copy, Clone)]
enum InputParameter {
    Value(i64),
    Position(usize),
    Relative(i64),
}

impl InputParameter {
    fn interpret(&self, memory: &Memory, base: usize) -> i64 {
        match *self {
            InputParameter::Value(value) => value,
            InputParameter::Position(pos) => memory.get(pos),
            InputParameter::Relative(offset) => memory.get((base as i64 + offset) as usize),
        }
    }
}

#[derive(Copy, Clone)]
enum OutputParameter {
    Position(usize),
    Relative(i64),
}

impl OutputParameter {
    fn absolute_position(&self, base: usize) -> usize {
        match *self {
            OutputParameter::Position(pos) => pos,
            OutputParameter::Relative(offset) => (base as i64 + offset) as usize,
        }
    }
}

impl CommandLike for Command {
    fn is_stop(&self) -> bool {
        match self {
            Stop => true,
            _ => false,
        }
    }

    fn command_length(&self) -> usize {
        match self {
            Stop => 1,
            Multiply(_, _, _) => 4,
            Add(_, _, _) => 4,
            Input(_) => 2,
            Output(_) => 2,
            JumpIfTrue(_, _) => 3,
            JumpIfFalse(_, _) => 3,
            LessThan(_, _, _) => 4,
            Equals(_, _, _) => 4,
            AdjustRelativeBase(_) => 2,
        }
    }

    fn execute<I: io::BufRead, O: io::Write>(
        &self,
        memory: &mut Memory,
        instruction_pointer: &mut isize,
        relative_base: &mut usize,
        input: &mut I,
        output: &mut O,
    ) {
        match *self {
            Stop => (),
            Add(parameter_a, parameter_b, dst) => {
                let value_a = parameter_a.interpret(memory, *relative_base);
                let value_b = parameter_b.interpret(memory, *relative_base);

                memory.put(dst.absolute_position(*relative_base), value_a + value_b);
            }
            Multiply(parameter_a, parameter_b, dst) => {
                let value_a = parameter_a.interpret(memory, *relative_base);
                let value_b = parameter_b.interpret(memory, *relative_base);

                memory.put(dst.absolute_position(*relative_base), value_a * value_b);
            }
            Input(dst) => loop {
                writeln!(output, "Request user input:");
                let mut line = String::new();
                input.read_line(&mut line);

                let parsed_result = line.trim().parse();

                if let Ok(value) = parsed_result {
                    memory.put(dst.absolute_position(*relative_base), value);
                    break;
                } else {
                    println!("Could not parse user input. Please type again.")
                }
            },
            Output(parameter) => {
                let value = parameter.interpret(memory, *relative_base);
                writeln!(output, "Output value: {}", value);
            }
            JumpIfFalse(condition, value) => Command::execute_jump(
                instruction_pointer,
                condition.interpret(memory, *relative_base),
                value.interpret(memory, *relative_base),
                |value| value == 0,
                self.command_length(),
            ),
            JumpIfTrue(condition, value) => Command::execute_jump(
                instruction_pointer,
                condition.interpret(memory, *relative_base),
                value.interpret(memory, *relative_base),
                |value| value != 0,
                self.command_length(),
            ),
            LessThan(a, b, dst) => memory.put(
                dst.absolute_position(*relative_base),
                Command::compare(
                    a.interpret(memory, *relative_base),
                    b.interpret(memory, *relative_base),
                    |a, b| a < b,
                ),
            ),
            Equals(a, b, dst) => memory.put(
                dst.absolute_position(*relative_base),
                Command::compare(
                    a.interpret(memory, *relative_base),
                    b.interpret(memory, *relative_base),
                    |a, b| a == b,
                ),
            ),
            AdjustRelativeBase(offset) => {
                *relative_base =
                    (*relative_base as i64 + offset.interpret(memory, *relative_base)) as usize
            }
        }
    }
}

trait CommandLike {
    fn is_stop(&self) -> bool;

    fn command_length(&self) -> usize;

    fn execute<I: io::BufRead, O: io::Write>(
        &self,
        memory: &mut Memory,
        instruction_pointer: &mut isize,
        relative_base: &mut usize,
        input: &mut I,
        output: &mut O,
    );
}

pub fn compute_memory_with_stdin_stdout(memory: Vec<i64>) -> (ComputationResult, Memory) {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut computer = IntComputer::new(memory, io::BufReader::new(stdin), &mut stdout);
    let result = computer.compute();

    (result, computer.memory)
}

impl<'a, I: io::BufRead, O: io::Write> IntComputer<'a, I, O> {
    pub fn new(memory: Vec<i64>, input: I, output: &'a mut O) -> IntComputer<I, O> {
        IntComputer {
            memory: Memory::new(memory),
            pointer: 0,
            relative_base: 0,
            input,
            output,
        }
    }

    pub fn compute(&mut self) -> ComputationResult {
        loop {
            let command_result = self.next_command();

            let optional_result = command_result
                .map(|command| {
                    command.execute(
                        &mut self.memory,
                        &mut self.pointer,
                        &mut self.relative_base,
                        &mut self.input,
                        &mut self.output,
                    );

                    self.pointer += command.command_length() as isize;

                    if command.is_stop() {
                        Some(Success)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|error| Some(Failure { error }));

            if let Some(result) = optional_result {
                return result;
            }
        }
    }

    pub fn memory(&self) -> &Vec<i64> {
        &self.memory.memory
    }

    fn next_command(&mut self) -> Result<Command, String> {
        let view = &self.memory[self.pointer as usize..];
        let opcode_with_modes = view[0];
        let parameters = &view[1..];
        let opcode = opcode_with_modes % 100;
        let modes = opcode_with_modes / 100;
        match opcode {
            1 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(Add(
                    input_parameters[0],
                    input_parameters[1],
                    parse_output_parameter(parameters[2], modes / 100),
                ))
            }
            2 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(Multiply(
                    input_parameters[0],
                    input_parameters[1],
                    parse_output_parameter(parameters[2], modes / 100),
                ))
            }
            3 => Ok(Input(parse_output_parameter(parameters[0], modes))),
            4 => {
                let input_parameters = parse_input_parameters(parameters, modes, 1);
                Ok(Output(input_parameters[0]))
            }
            5 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(JumpIfTrue(input_parameters[0], input_parameters[1]))
            }
            6 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(JumpIfFalse(input_parameters[0], input_parameters[1]))
            }
            7 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(LessThan(
                    input_parameters[0],
                    input_parameters[1],
                    parse_output_parameter(parameters[2], modes / 100),
                ))
            }
            8 => {
                let input_parameters = parse_input_parameters(parameters, modes, 2);
                Ok(Equals(
                    input_parameters[0],
                    input_parameters[1],
                    parse_output_parameter(parameters[2], modes / 100),
                ))
            }
            9 => {
                let input_parameters = parse_input_parameters(parameters, modes, 1);
                Ok(AdjustRelativeBase(input_parameters[0]))
            }
            99 => Ok(Stop),
            x => Err(format!("Could not parse command {}.", x)),
        }
    }
}

fn parse_output_parameter(value: i64, mode: i64) -> OutputParameter {
    let mode = mode % 10;

    if mode == 0 {
        OutputParameter::Position(value as usize)
    } else if mode == 2 {
        OutputParameter::Relative(value)
    } else {
        panic!("Unsupported mode for output parameter {}.", mode);
    }
}

fn parse_input_parameters(
    memory: &[i64],
    modes: i64,
    number_parameters: u32,
) -> Vec<InputParameter> {
    let mut result = Vec::new();
    let mut modes = modes;

    for idx in 0 as usize..number_parameters as usize {
        let mode = modes % 10;

        let parameter = if mode == 0 {
            InputParameter::Position(memory[idx] as usize)
        } else if mode == 1 {
            InputParameter::Value(memory[idx])
        } else if mode == 2 {
            InputParameter::Relative(memory[idx])
        } else {
            panic!("Unknown mode value: {}", mode);
        };

        result.push(parameter);
        modes /= 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_example_one() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        run_test(input, &output);
    }

    #[test]
    fn computes_example_two() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];
        run_test(input, &output);
    }

    #[test]
    fn computes_example_three() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];
        run_test(input, &output);
    }

    #[test]
    fn computes_example_four() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        run_test(input, &output);
    }

    fn run_test(input: Vec<i64>, output: &Vec<i64>) {
        let (result, resulting_memory) = compute_memory_with_stdin_stdout(input);
        assert_eq!(result, ComputationResult::Success);
        assert_eq!(resulting_memory.memory()[0..output.len()], output[..]);
    }

    #[test]
    fn computes_quine() {
        let memory = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let input: Vec<u8> = vec![];
        let mut output = vec![0; input.len()];
        let mut computer = IntComputer::new(memory.clone(), &input[..], &mut output);
        computer.compute();
        let result: Vec<i64> = String::from_utf8(output)
            .unwrap()
            .split("\n")
            .filter(|line| line.contains("Output value:"))
            .flat_map(|line: &str| {
                line.find(":")
                    .map(|idx| line[idx + 1..].trim().parse().unwrap())
            })
            .collect();

        assert_eq!(result, memory);
    }
}

impl Command {
    fn compare(a: i64, b: i64, comparison: fn(i64, i64) -> bool) -> i64 {
        if comparison(a, b) {
            1
        } else {
            0
        }
    }

    fn execute_jump(
        instruction_pointer: &mut isize,
        condition: i64,
        value: i64,
        jump_condition: fn(i64) -> bool,
        command_length: usize,
    ) -> () {
        if jump_condition(condition) {
            let new_instruction_pointer_value = value - command_length as i64;
            *instruction_pointer = new_instruction_pointer_value as isize;
        }
    }
}

pub fn read_memory_from_file(path: &str) -> Vec<i64> {
    let input = fs::read_to_string(path).unwrap();
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|split| i64::from_str(split).unwrap())
        .collect();
    memory
}
