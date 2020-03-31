pub struct FFT {

}

impl FFT {
    pub fn new() -> FFT {
        FFT {}
    }

    pub fn calculate(&self, input: &Vec<isize>, num_phases: usize) -> Vec<isize> {
        vec![]
    }
}

pub fn split_string_into_digits(content: String) -> Result<Vec<isize>, String> {
    const RADIX: u32 = 10;
    content
        .trim()
        .chars()
        .map(|chr| {
            chr.to_digit(RADIX)
                .map(|i| i as isize)
                .ok_or(format!("Could not parse char {} as digit.", chr))
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "80871224585914546619083218645595";
        let expected_output = "24176176";
        run_fft_test(input, expected_output);
    }

    #[test]
    fn example_two() {
        let input = "19617804207202209144916044189917";
        let expected_output = "73745418";
        run_fft_test(input, expected_output);
    }

    #[test]
    fn example_three() {
        let input = "69317163492948606335995924319873";
        let expected_output = "52432133";
        run_fft_test(input, expected_output);
    }

    fn run_fft_test(input: &str, expected_output: &str) {
        let input = split_string_into_digits(input.into()).unwrap();
        let fft = FFT::new();
        let output = fft.calculate(&input, 100);
        assert_eq!(output.iter().take(8).map(|&d| char::from(d as u8)).collect::<String>(), expected_output);
    }
}
