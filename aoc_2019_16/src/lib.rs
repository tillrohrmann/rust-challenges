type Element = i32;

pub struct FFT<'a> {
    input: &'a Vec<Element>,
}

impl<'a> FFT<'a> {
    const BASE_FREQUENCY: [Element; 4] = [0, 1, 0, -1];

    pub fn new(input: &'a Vec<Element>) -> FFT {
        FFT { input }
    }

    pub fn calculate(&self, num_phases: usize) -> Vec<Element> {
        let length = self.input.len();
        let mut result = self.input.clone();

        for i in 0..num_phases {
            let new_result = (0..length)
                .map(|idx| self.calculate_entry(&result, idx))
                .collect();

            result = new_result;
        }

        result
    }

    fn calculate_entry(&self, input: &Vec<Element>, idx: usize) -> Element {
        let mut result = 0;

        for (n, x) in input.iter().enumerate() {
            result += FFT::twiddle(n, idx) * x;
        }

        Element::abs(result % 10)
    }

    fn twiddle(n: usize, k: usize) -> Element {
        let remainder = (n + 1) / (k + 1) % 4;

        match remainder {
            0 | 2 => 0,
            1 => 1,
            3 => -1,
            _ => panic!("Illegal remainder")
        }
    }

    pub fn calculate_fast(&self, num_phases: usize) -> Vec<Element> {
        let length = self.input.len();
        let mut result = vec![0; length];
        let half = length / 2;

        let (_, right) = result.split_at_mut(half);
        right.copy_from_slice(&self.input[half..]);

        for _ in 0..num_phases {
            let mut sum = 0;

            for k in (half..length).rev() {
                sum = (sum + result[k]) % 10;
                result[k] = sum;
            }
        }

        result
    }
}

pub fn split_string_into_digits(content: String) -> Result<Vec<Element>, String> {
    const RADIX: u32 = 10;
    content
        .trim()
        .chars()
        .map(|chr| {
            chr.to_digit(RADIX)
                .map(|i| i as Element)
                .ok_or(format!("Could not parse char {} as digit.", chr))
        })
        .collect()
}

pub struct PartTwoSolver {
    input: Vec<Element>,
}

impl PartTwoSolver {
    pub fn new(input: &Vec<Element>, repetitions: usize) -> PartTwoSolver {
        let length = input.len();

        let mut repeated_input = Vec::with_capacity(length * repetitions);

        for _ in 0..repetitions {
            repeated_input.extend_from_slice(&input[..])
        }

        PartTwoSolver {
            input: repeated_input,
        }
    }

    pub fn calculate(&self, num_phases: usize) -> Vec<Element> {
        let fft = FFT::new(&self.input);
        let output = fft.calculate_fast(num_phases);
        let start = self
            .input
            .iter()
            .take(7)
            .fold(0, |acc, &value| acc * 10 + value) as usize;

        output[start..(start + 8)].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo_negative() {
        assert_eq!(4, isize::abs(-14 % 10));
    }

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

    #[test]
    fn part_2_example_one() {
        let input = "03036732577212944063491565474664";
        let expected_output = "84462026";

        run_part_2_fft_test(input, expected_output);
    }

    #[test]
    fn part_2_example_two() {
        let input = "02935109699940807407585447034323";
        let expected_output = "78725270";

        run_part_2_fft_test(input, expected_output);
    }

    #[test]
    fn part_2_example_three() {
        let input = "03081770884921959731165446850517";
        let expected_output = "53553731";

        run_part_2_fft_test(input, expected_output);
    }

    fn run_fft_test(input: &str, expected_output: &str) {
        let input = split_string_into_digits(input.into()).unwrap();
        let fft = FFT::new(&input);
        let output = fft.calculate(100);
        assert_eq!(
            output
                .iter()
                .take(8)
                .map(|&d| { d.to_string() })
                .collect::<String>(),
            expected_output
        );
    }

    fn run_part_2_fft_test(input: &str, expected_output: &str) {
        let input = split_string_into_digits(input.into()).unwrap();
        let solver = PartTwoSolver::new(&input, 10000);
        assert_eq!(
            solver
                .calculate(100)
                .iter()
                .map(|&d| d.to_string())
                .collect::<String>(),
            expected_output
        );
    }
}
