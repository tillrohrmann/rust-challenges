use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;

pub struct Recipes {
    recipes: Vec<usize>,
    counter: usize,
    elves: Vec<usize>,
}

impl Recipes {
    pub fn new() -> Recipes {
        Recipes {
            recipes: vec![3, 7],
            counter: 0,
            elves: vec![0, 1],
        }
    }

    pub fn find_recipes_after(&mut self, warmup_count: usize, recipe_count: usize) -> &[usize] {
        while self.recipes.len() < warmup_count + recipe_count {
            self.next_round();
        }

        &self.recipes[warmup_count..(warmup_count + recipe_count)]
    }

    fn next_round(&mut self) -> Vec<usize> {
        let mut current_sum: usize = self.elves.iter().map(|&i| self.recipes[i]).sum();
        let mut digits = VecDeque::with_capacity(2);

        let result = if current_sum == 0 {
            self.recipes.push(0);
            vec![0]
        } else {
            while current_sum > 0 {
                digits.push_back(current_sum % 10);
                current_sum = current_sum / 10;
            }

            let mut result = Vec::with_capacity(digits.len());
            while let Some(digit) = digits.pop_back() {
                self.recipes.push(digit);
                result.push(digit);
            }

            result
        };

        self.elves = self.elves.iter().map(|&i| (1 + self.recipes[i] + i) % self.recipes.len()).collect();

        result
    }

    pub fn number_recipes_until_sequence(&mut self, sequence: &[usize]) -> usize {
        let mut candidates = HashSet::new();
        while candidates.iter().filter(|Candidate(index, length)| *length == sequence.len()).count() == 0 {
            let length = self.recipes.len();
            let next_digits = self.next_round();

            candidates = candidates.into_iter().map(|Candidate(index, length)| {
                let suffix_length = std::cmp::min(next_digits.len(), sequence.len() - length);
                if sequence[length..(length + suffix_length)] == next_digits[0..suffix_length] {
                    Some(Candidate(index, length + suffix_length))
                } else {
                    None
                }
            }).flatten().collect();

            for i in 0..next_digits.len() {
                if next_digits[i..] == sequence[0..(next_digits.len() - i)] {
                    candidates.insert(Candidate(length + i, next_digits.len() - i));
                }
            }
        }

        let Candidate(index, length) = dbg!(candidates)
            .into_iter()
            .filter(|Candidate(index, length)| *length == sequence.len())
            .min_by(|Candidate(a, _), Candidate(b, _)| a.cmp(b))
            .unwrap();

        index
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Candidate(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_calculation_1() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.find_recipes_after(9, 10), &[5,1,5,8,9,1,6,7,7,9])
    }

    #[test]
    fn test_recipe_calculation_2() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.find_recipes_after(5, 10), &[0,1,2,4,5,1,5,8,9,1])
    }

    #[test]
    fn test_recipe_calculation_3() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.find_recipes_after(18, 10), &[9,2,5,1,0,7,1,0,8,5])
    }

    #[test]
    fn test_recipe_calculation_4() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.find_recipes_after(2018, 10), &[5,9,4,1,4,2,9,8,8,2])
    }

    #[test]
    fn test_recipes_until_sequence_1() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.number_recipes_until_sequence(&[5,1,5,8,9]), 9);
    }

    #[test]
    fn test_recipes_until_sequence_2() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.number_recipes_until_sequence(&[0,1,2,4,5]), 5);
    }

    #[test]
    fn test_recipes_until_sequence_3() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.number_recipes_until_sequence(&[9,2,5,1,0]), 18);
    }

    #[test]
    fn test_recipes_until_sequence_4() {
        let mut recipes = Recipes::new();
        assert_eq!(recipes.number_recipes_until_sequence(&[5,9,4,1,4]), 2018);
    }
}
