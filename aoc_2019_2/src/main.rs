use std::fs;
use std::str::FromStr;

use aoc_2019_2::ComputationResult::Success;
use aoc_2019_2::{
    compute_memory_with_stdin_stdout, read_memory_from_file, ComputationResult, IntComputer,
};

fn main() {
    solve_day_5_2();
}

fn test_input_equals_8() {
    let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let (result, _) = compute_memory_with_stdin_stdout(input);
    assert_eq!(result, ComputationResult::Success);
}

fn solve_day_5_2() {
    let memory = read_memory_from_file("input_day_5_2.txt");
    compute_result(memory);
}

fn solve_day_5() {
    let memory = read_memory_from_file("input_day_5.txt");
    compute_result(memory);
}

fn solve_day_2() {
    let memory = read_memory_from_file("input.txt");
    compute_result_with_modified_memory(&memory);
    find_verb_noun_for(&memory, 19690720);
}

fn compute_result_with_modified_memory(memory: &Vec<i64>) {
    let derived_memory = create_modified_memory(&memory, 12, 2);
    let result = compute_result(derived_memory);
    println!("{:?}", result);
}

fn compute_result(memory: Vec<i64>) -> i64 {
    let (result, resulting_memory) = compute_memory_with_stdin_stdout(memory);
    assert_eq!(result, ComputationResult::Success);
    resulting_memory[0]
}

fn create_modified_memory(original_input: &Vec<i64>, noun: i64, verb: i64) -> Vec<i64> {
    let mut result = original_input.clone();
    result[1] = noun;
    result[2] = verb;

    result
}

fn find_verb_noun_for(original_memory_input: &Vec<i64>, target_value: i64) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let modified_memory = create_modified_memory(original_memory_input, noun, verb);
            let (result, resulting_memory) = compute_memory_with_stdin_stdout(modified_memory);
            if result == Success && resulting_memory[0] == target_value {
                println!("Noun: {}, verb: {}.", noun, verb);
                return;
            }
        }
    }
}
