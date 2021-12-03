pub fn calculate_power_consumption(input: &Vec<&str>) -> u32 {
    let number_inputs = input.len();
    let counts_per_digit = count_per_digit(input);

    let values: Vec<u32> = counts_per_digit
        .iter()
        .map(|&value| {
            if (value as usize) > (number_inputs / 2) {
                1
            } else {
                0
            }
        })
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    for value in values {
        gamma <<= 1;
        epsilon <<= 1;
        if value == 1 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    gamma * epsilon
}

pub fn calculate_life_support(input: &Vec<&str>) -> u32 {
    let oxygen_rating = calculate_oxygen_rating(input);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(input);

    oxygen_rating * co2_scrubber_rating
}

fn calculate_co2_scrubber_rating(input: &Vec<&str>) -> u32 {
    calculate_rating(input, |value, len| (value * 2) < len)
}

fn calculate_rating<T>(input: &Vec<&str>, predicate: T) -> u32
where
    T: Fn(u32, u32) -> bool,
{
    let mut working_set = input.clone();
    let mut index = 0;

    while working_set.len() > 1 {
        let counts_per_digit = count_per_digit(&working_set);

        let mask: Vec<u32> = counts_per_digit
            .iter()
            .map(|&value| if predicate(value, working_set.len() as u32) { 1 } else { 0 })
            .collect();

        working_set = working_set.into_iter().filter(|&word| {
            map_to_int(word.chars().nth(index).unwrap()) == mask[index]
        }).collect();


        index += 1;
    }

    u32::from_str_radix(working_set[0], 2).unwrap()
}

fn calculate_oxygen_rating(input: &Vec<&str>) -> u32 {
    calculate_rating(input, |value, len| (value * 2) >= len)
}

fn count_per_digit(input: &Vec<&str>) -> Vec<u32> {
    let mut counts_per_digit: Vec<u32> = vec![0; input[0].len()];

    for &line in input {
        for (idx, value) in line.chars().map(|chr| map_to_int(chr)).enumerate() {
            counts_per_digit[idx] += value;
        }
    }
    counts_per_digit
}

fn map_to_int(chr: char) -> u32 {
    u32::from(chr) - u32::from('0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_life_support() {
        let input = get_input();
        assert_eq!(calculate_life_support(&input), 230);
    }

    #[test]
    fn test_calculate_power_consumption() {
        let input = get_input();
        assert_eq!(calculate_power_consumption(&input), 198);
    }

    fn get_input() -> Vec<&'static str> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
    }
}
