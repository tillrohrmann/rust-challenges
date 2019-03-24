pub struct FuelGrid {
    grid: Vec<Vec<isize>>,
    size: (usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct MaxPower(usize, usize, isize);

impl FuelGrid {
    pub fn new(width: usize, height: usize, serial_number: isize) -> FuelGrid {
        let mut grid = vec![vec![0; width]; height];

        for y in 0..height {
            for x in 0..width {
                let final_power_level = FuelGrid::calculate_power_level(serial_number, x as isize + 1, y as isize + 1);

                grid[y][x] = final_power_level;
            }
        }

        FuelGrid {
            grid,
            size: (width, height),
        }
    }

    fn calculate_power_level(serial_number: isize, x: isize, y: isize) -> isize {
        let rack_id = x as isize + 10;
        let power_level = rack_id * (y as isize) + serial_number;
        let power_level_rack_id = power_level * rack_id;
        let final_power_level = (power_level_rack_id / 100) % 10 - 5;
        final_power_level
    }

    pub fn max_power_square(&self, w: usize, h: usize) -> MaxPower {
        let mut max_power = isize::min_value();
        let mut max_x = 0;
        let mut max_y = 0;
        let (width, height) = self.size;

        for y in 0..(height - h + 1) {
            for x in 0..(width - w + 1) {
                let mut power = 0;
                for p_y in 0..h {
                    for p_x in 0..w {
                        power += self.grid[y + p_y][x + p_x];
                    }
                }

                if power > max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                }
            }
        }

        MaxPower(max_x + 1, max_y + 1, max_power)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level_calculation_1() {
        assert_eq!(FuelGrid::calculate_power_level(57, 122, 79), -5);
    }

    #[test]
    fn test_power_level_calculation_2() {
        assert_eq!(FuelGrid::calculate_power_level(39, 217, 196), 0);
    }

    #[test]
    fn test_power_level_calculation_3() {
        assert_eq!(FuelGrid::calculate_power_level(71, 101, 153), 4);
    }

    #[test]
    fn test_max_power_square() {
        let grid = FuelGrid::new(300, 300, 18);
        assert_eq!(grid.max_power_square(3, 3), MaxPower(33, 45, 29));
    }
}
