use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_two(input: &String) {
    let result = aoc_2021_4::play_losing_game(&input.split("\n").collect()).unwrap();
    println!("Solve part two: {}", result);
}

fn solve_part_one(input: &String) {
    let result = aoc_2021_4::play_game(&input.split("\n").collect()).unwrap();
    println!("Solve part one: {}", result);
}
