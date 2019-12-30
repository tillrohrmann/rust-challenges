use std::{fs, io};

fn main() {
    let input: Vec<String> = read_lines_of_file("input.txt").unwrap();

    let minimal_distance =
        aoc_2019_3::calculate_minimal_distance_intersections(&input[0], &input[1]).unwrap();

    let minimal_steps = aoc_2019_3::calculate_minimal_steps_intersections(&input[0], &input[1]).unwrap();

    println!("Minimal manhattan distance: {}, minimal steps: {}", minimal_distance, minimal_steps);
}

fn read_lines_of_file(filename: &str) -> io::Result<Vec<String>> {
    let file = fs::read_to_string(filename)?;

    let result: Vec<String> = file.split("\n").map(|line| line.to_string()).collect();

    Ok(result)
}
