pub struct Permutate<'a, T: 'a> {
    input: &'a Vec<T>,
    counter: usize,
    number_permutations: usize,
}

fn factorial(v: usize) -> usize {
    (1..=v).fold(1, |result, value| result * value)
}

impl<'a, T> Permutate<'a, T> {
    pub fn new(input: &'a Vec<T>) -> Permutate<'a, T> {
        let number_permutations = factorial(input.len());

        Permutate {
            input,
            number_permutations,
            counter: 0,
        }
    }
}

impl<'a, T> Iterator for Permutate<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.counter >= self.number_permutations {
            None
        } else {
            let mut result: Vec<&'a T> = Vec::with_capacity(self.input.len());

            for element in self.input {
                result.push(element);
            }

            let mut swaps = self.counter;
            let mut index = 0;
            let len = result.len();

            while swaps > 0 {
                let base = len - index;
                let target = swaps % base;

                if target > 0 {
                    let element = result[index];
                    result[index] = result[target + index];
                    result[target + index] = element;
                }

                swaps /= base;
                index += 1;
            }

            self.counter += 1;

            Some(result)
        }
    }
}

pub fn is_valid_configuration(input: &Vec<&i32>) -> bool {
    assert_eq!(input.len() % 2, 0);

    let max_index = input.len() / 2;
    let mut index = 1;
    let start_value = *input[0];

    while index < max_index && start_value < *input[index] {
        index += 1;
    }

    if index == max_index {
        index = 1;
        let mut sum = calculate_arm(input, 0);

        while index < max_index && sum == calculate_arm(input, index) {
            index += 1;
        }

        index == max_index
    } else {
        false
    }
}

fn calculate_arm(input: &Vec<&i32>, start: usize) -> i32 {
    assert_eq!(input.len() % 2, 0);

    let half = input.len() / 2;

    assert!(start < half);
    let outer_index = start;
    let inner_index = start;
    input[outer_index] + input[half + (inner_index)] + input[half + (inner_index + 1) % half]
}

pub fn concatenate_permutation(input: &Vec<&i32>) -> i64 {
    assert_eq!(input.len() % 2, 0);
    let half = input.len() / 2;
    let mut result = 0;

    for i in 0..half {
        result = add_number(result, *input[i]);
        result = add_number(result, *input[half + i]);
        result = add_number(result, *input[half + (i + 1) % half]);
    }

    result
}

pub fn count_digits(mut input: i64) -> u32 {
    let mut result = 0;
    while input > 0 {
        result += 1;
        input /= 10;
    }

    result
}

fn add_number(result: i64, number: i32) -> i64 {
    let mut shift = 1;
    let mut current_number = number;

    while current_number > 0 {
        shift *= 10;
        current_number /= 10;
    }

    result * shift + number as i64
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_concatenate_permutation() {
        let input = vec![&4, &6, &5, &3, &2, &1];
        assert_eq!(concatenate_permutation(&input), 432621513)
    }

    #[test]
    fn test_calculate_arm() {
        let input = vec![&4, &6, &5, &3, &2, &1];

        assert_eq!(calculate_arm(&input, 0), 9)
    }

    #[test]
    fn test_is_valid_configuration() {
        let input = vec![&4, &6, &5, &3, &2, &1];
        assert!(is_valid_configuration(&input))
    }

    #[test]
    fn test_simple_permutation() {
        let input = vec![1];
        let permutate = Permutate::new(&input);

        assert_eq!(permutate.collect::<HashSet<Vec<&i32>>>(), vec![vec![&1]].iter().cloned().collect())
    }

    #[test]
    fn test_two_element_permutation() {
        let input = vec![1, 2];
        let permutate = Permutate::new(&input);
        assert_eq!(permutate.collect::<HashSet<Vec<&i32>>>(), vec![vec![&1, &2], vec![&2, &1]].iter().cloned().collect())
    }

    #[test]
    fn test_string_permutation() {
        let input = vec!["1", "2", "3"];
        let permutate = Permutate::new(&input);
        let expected = vec![vec![&"1", &"2", &"3"], vec![&"1", &"3", &"2"], vec![&"2", &"1", &"3"], vec![&"2", &"3", &"1"], vec![&"3", &"1", &"2"], vec![&"3", &"2", &"1"]].iter().cloned().collect();
        assert_eq!(permutate.collect::<HashSet<Vec<&&str>>>(), expected)
    }
}
