struct Permutate<T> {
    input: Vec<T>,
}

impl<T> Permutate<T> {
    fn new(input: Vec<T>) -> Permutate<T> {
        Permutate {
            input
        }
    }
}

impl<T> Iterator for Permutate<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_permutation() {
        let input = vec![1];
        let permutate = Permutate::new(input);

        assert_eq!(permutate.collect::<Vec<Vec<i32>>>(), vec![vec![1]])
    }

    fn test_two_element_permutation() {
        let input = vec![1, 2];
        let permutate = Permutate::new(input);
        assert_eq!(permutate.collect::<Vec<Vec<i32>>>(), vec![vec![1, 2], vec![2, 1]])
    }

    fn test_string_permutation() {
        let input = vec!["1", "2", "3"];
        let permutate = Permutate::new(input);
        let expected = vec![vec!["1", "2", "3"], vec!["1", "3", "2"], vec!["2", "1", "3"], vec!["2", "3", "1"], vec!["3", "1", "2"], vec!["3", "2", "1"]];
        assert_eq!(permutate.collect::<Vec<Vec<&str>>>(), expected)
    }
}
