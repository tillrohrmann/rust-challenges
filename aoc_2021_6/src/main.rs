use aoc_common::GenericResult;
use std::num::ParseIntError;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let initial_fishes = parse_content(&content).unwrap();

    solve_part_one(&initial_fishes);
    solve_part_two(&initial_fishes);
}

fn solve_part_one(initial_fishes: &Vec<u32>) {
    let result = aoc_2021_6::calculate_fishes_for(&initial_fishes, 80);

    println!("Part one: {}.", result);
}

fn solve_part_two(initial_fishes: &Vec<u32>) {
    let result = aoc_2021_6::calculate_fishes_for(&initial_fishes, 256);

    println!("Part two: {}.", result);
}

fn parse_content(input: &Vec<String>) -> GenericResult<Vec<u32>> {
    input
        .iter()
        .flat_map(|line| line.split(','))
        .map(|number| number.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(|err| err.into())
}
