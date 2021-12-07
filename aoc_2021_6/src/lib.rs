#[derive(Debug)]
struct Fishes {
    fishes: [u32; 9],
    day: usize,
}

impl Fishes {
    fn new(initial_fishes: &Vec<u32>) -> Fishes {
        let mut fishes = [0; 9];

        for &fish in initial_fishes {
            assert!(fish <= 8);
            fishes[fish as usize] += 1;
        }

        Fishes { fishes, day: 0 }
    }

    fn simulate_single_day(&mut self) {
        let fishes_7_day = self.getFishes(7);
        let fishes_0_day = self.getFishes(0);

        self.next_day();

        self.addFishes(6, fishes_7_day);
        self.setFishes(8, fishes_0_day);
    }

    fn simulate_days(&mut self, days: u32) {
        for _ in 0..days {
            self.simulate_single_day();
        }
    }

    fn number_fishes(&self) -> u32 {
        self.fishes.iter().sum()
    }

    fn getFishes(&self, day: usize) -> u32 {
        let index = self.get_index(day);

        self.fishes[index]
    }

    fn addFishes(&mut self, day: usize, increment: u32) {
        let index = self.get_index(day);

        self.fishes[index] += increment;
    }

    fn setFishes(&mut self, day: usize, new_value: u32) {
        let index = self.get_index(day);
        self.fishes[index] = new_value;
    }

    fn get_index(&self, day: usize) -> usize {
        if day <= 6 {
            (self.day + day) % 7
        } else {
            (self.day + day) % 2 + 7
        }
    }
    fn next_day(&mut self) {
        self.day += 1;
    }
}

pub fn calculate_fishes_for(initial_fishes: &Vec<u32>, days: u32) -> u32 {
    let mut fishes = Fishes::new(initial_fishes);
    fishes.simulate_days(days);
    fishes.number_fishes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fishes() {
        let initial_fishes = vec![3, 4, 3, 1, 2];
        assert_eq!(calculate_fishes_for(&initial_fishes, 80), 5934);
    }
}
