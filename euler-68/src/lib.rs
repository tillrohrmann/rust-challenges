struct Permutate<'a, T: 'a> {
    input: &'a Vec<T>,
    counter: usize,
    number_permutations: usize,
}

fn factorial(v: usize) -> usize {
    (1..=v).fold(1, |result, value| result * value)
}

impl<'a, T> Permutate<'a, T> {
    fn new(input: &'a Vec<T>) -> Permutate<'a, T> {
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

            self.counter += 1;

            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_permutation() {
        let input = vec![1];
        let permutate = Permutate::new(&input);

        assert_eq!(permutate.collect::<Vec<Vec<&i32>>>(), vec![vec![&1]])
    }

    #[test]
    fn test_two_element_permutation() {
        let input = vec![1, 2];
        let permutate = Permutate::new(&input);
        assert_eq!(permutate.collect::<Vec<Vec<&i32>>>(), vec![vec![&1, &2], vec![&2, &1]])
    }

    #[test]
    fn test_string_permutation() {
        let input = vec!["1", "2", "3"];
        let permutate = Permutate::new(&input);
        let expected = vec![vec![&"1", &"2", &"3"], vec![&"1", &"3", &"2"], vec![&"2", &"1", &"3"], vec![&"2", &"3", &"1"], vec![&"3", &"1", &"2"], vec![&"3", &"2", &"1"]];
        assert_eq!(permutate.collect::<Vec<Vec<&&str>>>(), expected)
    }
}
