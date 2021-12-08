use std::cmp::min;

struct Crabs {
    number_per_position: Vec<u32>,
}

impl Crabs {
    fn new(initial_positions: &Vec<u32>) -> Crabs {
        let max_value = initial_positions.iter().max().cloned().unwrap_or(0) as usize;

        let mut number_per_position = vec![0; max_value + 1];

        for &position in initial_positions {
            number_per_position[position as usize] += 1;
        }

        Crabs {
            number_per_position,
        }
    }

    fn calculate_minimal_fuel(&self) -> usize {
        let (mut right_side, mut current_fuel) = self.number_crabs_and_fuel();
        let mut min_fuel = current_fuel;
        let mut min_position = usize::MAX;
        let mut left_side = 0;

        for index in 0..self.number_per_position.len() {
            current_fuel -= right_side as usize;
            current_fuel += left_side as usize;

            right_side -= self.number_per_position[index];
            left_side += self.number_per_position[index];

            if current_fuel < min_fuel {
                min_fuel = current_fuel;
                min_position = index;
            }
        }

        min_fuel
    }

    fn number_crabs_and_fuel(&self) -> (u32, usize) {
        let mut total_crabs = 0;
        let mut fuel = 0;

        for (position, &crabs) in self.number_per_position.iter().enumerate() {
            total_crabs += crabs;
            fuel += (position + 1) * crabs as usize;
        }

        (total_crabs, fuel)
    }

    pub fn calculate_minimal_fuel_with_increasing_rate(&self) -> usize {
        let mut min_fuel = usize::MAX;
        let mut min_position = usize::MAX;

        for position in 0..self.number_per_position.len() {
            let mut current_fuel = 0;

            for left in 0..position {
                let n = position - left;
                current_fuel +=
                    self.number_per_position[left] as usize * ((n + 1) * n / 2 as usize);
            }

            for right in (position + 1)..self.number_per_position.len() {
                let n = right - position;
                current_fuel +=
                    self.number_per_position[right] as usize * ((n + 1) * n / 2 as usize);
            }

            if current_fuel < min_fuel {
                min_fuel = current_fuel;
                min_position = position;
            }
        }

        min_fuel
    }
}

pub fn calculate_minimal_fuel(initial_positions: &Vec<u32>) -> usize {
    let crabs = Crabs::new(initial_positions);

    crabs.calculate_minimal_fuel()
}

pub fn calculate_minimal_fuel_with_increasing_rate(initial_positions: &Vec<u32>) -> usize {
    let crabs = Crabs::new(initial_positions);

    crabs.calculate_minimal_fuel_with_increasing_rate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(calculate_minimal_fuel(&input), 37);
    }

    #[test]
    fn calculate_fuel_with_increasing_rate() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(calculate_minimal_fuel_with_increasing_rate(&input), 168);
    }
}
