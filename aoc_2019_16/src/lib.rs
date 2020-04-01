pub struct FFT<'a> {
    input: &'a Vec<isize>,
    frequencies: Vec<Vec<isize>>,
}

impl<'a> FFT<'a> {
    const BASE_FREQUENCY: [isize; 4] = [0, 1, 0, -1];

    pub fn new(input: &'a Vec<isize>) -> FFT {
        let frequencies = FFT::calculate_frequencies(input.len());

        FFT { input, frequencies }
    }

    fn calculate_frequencies(number_frequencies: usize) -> Vec<Vec<isize>> {
        (0..number_frequencies)
            .map(|frequency| FFT::calculate_frequency(frequency, number_frequencies))
            .collect()
    }

    fn calculate_frequency(frequency: usize, length: usize) -> Vec<isize> {
        (0..length)
            .map(|idx| FFT::calculate_frequency_entry(idx, frequency))
            .collect()
    }

    fn calculate_frequency_entry(idx: usize, frequency: usize) -> isize {
        let period = FFT::BASE_FREQUENCY.len() * (frequency + 1);
        let period_idx = (idx + 1) % period;
        let base_period_idx = period_idx / (frequency + 1);

        FFT::BASE_FREQUENCY[base_period_idx]
    }

    pub fn calculate(&self, num_phases: usize) -> Vec<isize> {
        let length = self.input.len();
        let mut result = self.input.clone();

        for _ in 0..num_phases {
            let new_result = (0..length)
                .map(|idx| self.calculate_entry(&result, idx))
                .collect();

            result = new_result;
        }

        result
    }

    fn calculate_entry(&self, input: &Vec<isize>, idx: usize) -> isize {
        let result: isize = input
            .iter()
            .zip(self.get_frequency(idx))
            .map(|(a, b)| a * b)
            .sum();

        isize::abs(result % 10)
    }

    fn get_frequency(&self, idx: usize) -> &Vec<isize> {
        self.frequencies.get(idx).unwrap()
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
    fn frequency_calculation_one() {
        let frequency = FFT::calculate_frequency(0, 8);

        assert_eq!(frequency, vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn frequency_calculation_two() {
        let frequency = FFT::calculate_frequency(1, 8);

        assert_eq!(frequency, vec![0, 1, 1, 0, 0, -1, -1, 0]);
    }

    #[test]
    fn frequency_calculation_three() {
        let frequency = FFT::calculate_frequency(2, 8);

        assert_eq!(frequency, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }

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

    fn run_fft_test(input: &str, expected_output: &str) {
        let input = split_string_into_digits(input.into()).unwrap();
        let fft = FFT::new(&input);
        let output = fft.calculate(100);
        assert_eq!(
            output
                .iter()
                .take(8)
                .map(|&d| {
                   d.to_string()
                })
                .collect::<String>(),
            expected_output
        );
    }
}
