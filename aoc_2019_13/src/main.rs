use aoc_2019_2::IntComputer;
use std::num::ParseIntError;

fn main() {
    let memory = aoc_2019_2::read_memory_from_file("input.txt");

    let mut output = Vec::new();
    let mut computer = IntComputer::new(memory, "".as_bytes(), &mut output);
    computer.compute();
    let output = String::from_utf8(output).unwrap();

    let result: Vec<u32> = output
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line: &str| {
            line.trim_start_matches("Output value: ").trim().parse()
        })
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .unwrap();

    let block_tiles = result.chunks(3).filter(|&chunk| chunk[2] == 2).count();
    println!("{}", block_tiles);
}
