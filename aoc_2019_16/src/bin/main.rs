use std::fs;

fn main() {
    solve_part_2();
}

fn solve_part_2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let digits = aoc_2019_16::split_string_into_digits(content).unwrap();
    let solver = aoc_2019_16::PartTwoSolver::new(&digits, 10000);
    let result = solver.calculate(100);

    println!("{:?}", result);
}

fn solve_part_1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let digits = aoc_2019_16::split_string_into_digits(content).unwrap();
    let fft = aoc_2019_16::FFT::new(&digits);
    let output = fft.calculate(100);
    println!("{:?}", output);
}
