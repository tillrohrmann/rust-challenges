use aoc_2019_2::{IntComputer, ComputationResult};
use std::fs;
use std::str::FromStr;
use aoc_2019_2::ComputationResult::Success;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let memory: Vec<u32> = input
        .trim()
        .split(',')
        .map(|split| u32::from_str(split).unwrap())
        .collect();
    compute_result(&memory);
    find_verb_noun_for(&memory, 19690720);
}

fn compute_result(memory: &Vec<u32>) {
    let derived_memory = create_modified_memory(&memory, 12, 2);
    let mut computer = IntComputer::new(derived_memory);
    assert_eq!(computer.compute(), ComputationResult::Success);
    println!("{:?}", computer.memory()[0]);
}

fn create_modified_memory(original_input: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {
    let mut result = original_input.clone();
    result[1] = noun;
    result[2] = verb;

    result
}

fn find_verb_noun_for(original_memory_input: &Vec<u32>, target_value: u32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let modified_memory = create_modified_memory(original_memory_input, noun, verb);
            let mut computer = IntComputer::new(modified_memory);
            if computer.compute() == Success && computer.memory()[0] == target_value {
                println!("Noun: {}, verb: {}.", noun, verb);
                return;
            }
        }
    }
}
