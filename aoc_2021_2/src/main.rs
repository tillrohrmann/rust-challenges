use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.trim().split("\n").map(|line| line.trim()).collect();

    solve_part_one(&lines);
    solve_part_two(&lines);
}

fn solve_part_one(lines: &Vec<&str>) {
    let position = aoc_2021_2::simulate_submarine(lines).unwrap();

    println!("Position {:?}, result {}", position, position.x * position.y);
}

fn solve_part_two(lines: &Vec<&str>) {
    let position = aoc_2021_2::simulate_submarine_aim(lines).unwrap();

    println!("Position {:?}, result {}", position, position.x * position.y);
}