use std::fs;

fn main() {
    const RADIX: u32 = 10;
    let content = fs::read_to_string("input.txt").unwrap();
    let input: Result<Vec<isize>, String> = content
        .trim()
        .chars()
        .map(|chr| {
            chr.to_digit(RADIX)
                .map(|i| i as isize)
                .ok_or(format!("Could not parse char {} as digit.", chr))
        })
        .collect();

    let input = input.unwrap();
    let fft = aoc_2019_16::FFT::new();

    let output = fft.calculate(&input);

    println!("{:?}", output);
}
