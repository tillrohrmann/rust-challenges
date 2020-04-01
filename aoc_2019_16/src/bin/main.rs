use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let digits = aoc_2019_16::split_string_into_digits(content).unwrap();

    let fft = aoc_2019_16::FFT::new(&digits);

    let output = fft.calculate(100);

    println!("{:?}", output);
}
