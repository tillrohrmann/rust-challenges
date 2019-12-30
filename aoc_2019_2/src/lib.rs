use crate::ComputationResult::{Success, Failure};
use crate::Command::{*};
use std::io;

pub struct IntComputer {
    memory: Vec<i32>,
    pointer: usize,
}

#[derive(PartialEq, Debug)]
pub enum ComputationResult {
    Success,
    Failure { error: String },
}

enum Command {
    Add(InputParameter, InputParameter, usize),
    Multiply(InputParameter, InputParameter, usize),
    Input(usize),
    Output(InputParameter),
    Stop,
}

#[derive(Copy, Clone)]
enum InputParameter {
    Value(i32),
    Position(usize),
}

impl InputParameter {
    fn interpret(&self, memory: &mut Vec<i32>) -> i32 {
        match *self {
            InputParameter::Value(value) => value,
            InputParameter::Position(pos) => memory[pos],
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
        }
    }

    fn execute(&self, memory: &mut Vec<i32>) {
        match *self {
            Stop => (),
            Add(parameter_a, parameter_b, dst) => {
                let value_a = parameter_a.interpret(memory);
                let value_b = parameter_b.interpret(memory);

                memory[dst] = value_a + value_b;
            },
            Multiply(parameter_a, parameter_b, dst) => {
                let value_a = parameter_a.interpret(memory);
                let value_b = parameter_b.interpret(memory);
                memory[dst] = value_a * value_b;
            },
            Input(dst) => {
                loop {
                    println!("Request user input:");
                    let mut line = String::new();
                    io::stdin().read_line(&mut line);

                    let parsed_result = line.trim().parse();

                    if let Ok(value) = parsed_result {
                        memory[dst] = value;
                        break;
                    } else {
                        println!("Could not parse user input. Please type again.")
                    }
                }
            },
            Output(parameter) => {
                let value = parameter.interpret(memory);
                println!("Output value: {}", value);
            }
        }
    }
}

trait CommandLike {
    fn is_stop(&self) -> bool;

    fn command_length(&self) -> usize;

    fn execute(&self, memory: &mut Vec<i32>);
}

impl IntComputer {
    pub fn new(memory: Vec<i32>) -> IntComputer {
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

    pub fn memory(&self) -> &Vec<i32> {
        &self.memory
    }

    fn next_command(&mut self) -> Result<Command, String> {
        let view = &self.memory[self.pointer as usize..];
        let opcode_with_modes = view[0];
        let parameters = &view[1..];
        let opcode = opcode_with_modes % 100;
        let modes = opcode_with_modes / 100;
        match opcode {
            1 => {
                let input_parameters = IntComputer::parse_input_parameters(parameters, modes, 2);
                Ok(Add(input_parameters[0], input_parameters[1], parameters[2] as usize))
            },
            2 => {
                let input_parameters = IntComputer::parse_input_parameters(parameters, modes, 2);
                Ok(Multiply(input_parameters[0], input_parameters[1], parameters[2] as usize))
            },
            3 => {
                Ok(Input(parameters[0] as usize))
            },
            4 => {
                let input_parameters = IntComputer::parse_input_parameters(parameters, modes, 1);
                Ok(Output(input_parameters[0]))
            }
            99 => {
                Ok(Stop)
            },
            x => Err(format!("Could not parse command {}.", x))
        }
    }

    fn parse_input_parameters(memory: &[i32], modes: i32, number_parameters: u32) -> Vec<InputParameter> {
        let mut result = Vec::new();
        let mut modes = modes;

        for idx in 0 as usize..number_parameters as usize {
            let mode = modes % 10;

            let parameter = if mode == 0 {
                InputParameter::Position(memory[idx] as usize)
            } else if mode == 1 {
                InputParameter::Value(memory[idx])
            } else {
                panic!("Unknown mode value: {}", mode);
            };

            result.push(parameter);
            modes /= 10;
        }

        result
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

    fn run_test(input: Vec<i32>, output: &Vec<i32>) {
        let mut computer = IntComputer::new(input);
        assert_eq!(computer.compute(), ComputationResult::Success);
        assert_eq!(computer.memory(), output);
    }
}
