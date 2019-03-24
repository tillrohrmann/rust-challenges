use aoc11::{FuelGrid, MaxPower};

fn main() {
    let grid = FuelGrid::new(300, 300, 9221);
    let result = (1..=300).map(|size| grid.max_power_square(size, size)).max_by(|&MaxPower(_, _, _, a_power), &MaxPower(_, _, _, b_power)| a_power.cmp(&b_power)).unwrap();
    println!("{:?}", result);
}