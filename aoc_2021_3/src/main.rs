use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.trim().split("\n").map(|line| line.trim()).collect();

    solve_part_one(&lines);
    solve_part_two(&lines);
}

fn solve_part_two(lines: &Vec<&str>) {
    let life_support = aoc_2021_3::calculate_life_support(lines);

    println!("Solve part two: {}", life_support);
}

fn solve_part_one(lines: &Vec<&str>) {
    let power_consumption = aoc_2021_3::calculate_power_consumption(&lines);

    println!("Solve part one: {}", power_consumption);
}