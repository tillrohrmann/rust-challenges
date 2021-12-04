use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = aoc_2021_4::play_game(&input.split("\n").collect()).unwrap();

    println!("Solve part one: {}", result);
}
