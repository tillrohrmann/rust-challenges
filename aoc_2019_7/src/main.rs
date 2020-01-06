use core::mem;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::process::Output;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::thread::JoinHandle;

use aoc_2019_2::{read_memory_from_file, ComputationResult, IntComputer};
use permutohedron::LexicalPermutation;

use aoc_2019_7::{ChannelReader, ChannelWriter};

fn main() {
    solve_day_7_2()
}

fn solve_day_7_1() {
    let result = aoc_2019_7::find_largest_permutation(
        read_memory_from_file("input_day_7.txt"),
        vec![0, 1, 2, 3, 4],
    );
    println!("{}", result.unwrap());
}

fn solve_day_7_2() {
    let result = aoc_2019_7::find_largest_permutation_for_feedback_sequence(
        read_memory_from_file("input_day_7.txt"),
        (5..=9).collect::<Vec<i32>>()
    );
    println!("{}", result.unwrap());
}
