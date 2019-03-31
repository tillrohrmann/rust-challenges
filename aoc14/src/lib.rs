use std::collections::vec_deque::VecDeque;

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
            let mut current_sum: usize = self.elves.iter().map(|&i| self.recipes[i]).sum();

            let mut digits = VecDeque::with_capacity(2);

            if current_sum == 0 {
                self.recipes.push(0);
            } else {
                while current_sum > 0 {
                    digits.push_back(current_sum % 10);
                    current_sum = current_sum / 10;
                }

                while let Some(digit) = digits.pop_back() {
                    self.recipes.push(digit);
                }
            }

            self.elves = self.elves.iter().map(|&i| (1 + self.recipes[i] + i) % self.recipes.len()).collect();

        }

        &self.recipes[warmup_count..(warmup_count + recipe_count)]
    }
}

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
}
