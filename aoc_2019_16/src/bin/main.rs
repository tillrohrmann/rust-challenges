use std::fs;

fn main() {
    solve_part_2();
}

fn solve_part_2() {
    let content = "03036732577212944063491565474664".into();
    let digits = aoc_2019_16::split_string_into_digits(content).unwrap();
    let fft = aoc_2019_16::FFT::new(&digits);
    let output = fft.calculate(100);
    let output: String = output.iter().map(|x| x.to_string()).collect();

    println!("{}", output)
}

fn solve_part_1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let digits = aoc_2019_16::split_string_into_digits(content).unwrap();
    let fft = aoc_2019_16::FFT::new(&digits);
    let output = fft.calculate(100);
    println!("{:?}", output);
}
