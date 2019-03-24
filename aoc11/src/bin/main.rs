use aoc11::FuelGrid;

fn main() {
    let grid = FuelGrid::new(300, 300, 9221);
    println!("{:?}", grid.max_power_square(3, 3));
}