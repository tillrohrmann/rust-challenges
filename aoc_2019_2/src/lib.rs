use crate::ComputationResult::{Success, Failure};
use crate::Command::{Stop, Add, Multiply};

pub struct IntComputer {
    memory: Vec<u32>,
    pointer: usize,
}

#[derive(PartialEq, Debug)]
pub enum ComputationResult {
    Success,
    Failure { error: String },
}

enum Command {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Stop,
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
        }
    }

    fn execute(&self, memory: &mut Vec<u32>) {
        match *self {
            Stop => (),
            Add(src_a, src_b, dst) => memory[dst] = memory[src_a] + memory[src_b],
            Multiply(src_a, src_b, dst) => memory[dst] = memory[src_a] * memory[src_b],
        }
    }
}

trait CommandLike {
    fn is_stop(&self) -> bool;

    fn command_length(&self) -> usize;

    fn execute(&self, memory: &mut Vec<u32>);
}

impl IntComputer {
    pub fn new(memory: Vec<u32>) -> IntComputer {
        IntComputer { memory, pointer: 0 }
    }

    pub fn compute(&mut self) -> ComputationResult {
        loop {
            let command_result = self.next_command();

            let optional_result = command_result.map(|command| {
                command.execute(&mut self.memory);
                self.pointer += command.command_length();

                if command.is_stop() {
                    Some(Success)
                } else {
                    None
                }
            }).unwrap_or_else(|error| Some(Failure{ error }));

            if let Some(result) = optional_result {
                return result;
            }
        }
    }

    pub fn memory(&self) -> &Vec<u32> {
        &self.memory
    }

    fn next_command(&mut self) -> Result<Command, String> {
        let view = &self.memory[self.pointer as usize..];
        match view[0] {
            1 => {
                Ok(Add(view[1] as usize, view[2] as usize, view[3] as usize))
            },
            2 => {
                Ok(Multiply(view[1] as usize, view[2] as usize, view[3] as usize))
            },
            99 => {
                Ok(Stop)
            },
            x => Err(format!("Could not parse command {}.", x))
        }
    }
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

    fn run_test(input: Vec<u32>, output: &Vec<u32>) {
        let mut computer = IntComputer::new(input);
        assert_eq!(computer.compute(), ComputationResult::Success);
        assert_eq!(computer.memory(), output);
    }
}
