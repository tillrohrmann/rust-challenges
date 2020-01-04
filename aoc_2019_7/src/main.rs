use aoc_2019_2::{read_memory_from_file, ComputationResult, IntComputer};
use core::mem;
use permutohedron::LexicalPermutation;

fn main() {
    solve_day_7_1()
}

fn solve_day_7_1() {
    let result = aoc_2019_7::find_largest_permutation(
        read_memory_from_file("input_day_7.txt"),
        vec![0, 1, 2, 3, 4],
    );
    println!("{}", result.unwrap());
}
