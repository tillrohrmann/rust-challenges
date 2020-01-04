use aoc_2019_2::{read_memory_from_file, ComputationResult, IntComputer};
use core::mem;
use permutohedron::LexicalPermutation;

fn main() {
    solve_day_7_1()
}

fn solve_day_7_1() {
    let result = find_largest_permutation();
    println!("{}", result.unwrap());
}

fn find_largest_permutation() -> Result<i32, String> {
    let memory = read_memory_from_file("input_day_7.txt");
    let mut input: Vec<i32> = vec![0, 1, 2, 3, 4];
    let mut results = Vec::new();

    loop {
        results.push(compute_result_for_sequence(&memory, &input)?);

        if !input.next_permutation() {
            break;
        }
    }

    results.into_iter().max().ok_or("No valid value found.".to_string())
}

fn compute_result_for_sequence(memory: &Vec<i32>, sequence: &Vec<i32>) -> Result<i32, String> {
    let mut result = 0;

    for &first_input in sequence {
        result = compute_result(memory.clone(), first_input, result)?
    }

    Ok(result)
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
        .flat_map(|line: &str| line.find(":").map(|idx| {
            line[idx + 1..].trim().parse().map_err(|err| format!("Could not parse integer: {}", err))
        }))
        .collect();

    let result = mem::replace(&mut x[0], Ok(42));

    result
}
