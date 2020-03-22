use aoc_2019_12::{Moon, MoonSimulator};
use aoc_common::math::Point3d;

fn main() {
    solve_day_12_2()
}

fn create_simple_moons() -> Vec<Moon> {
    let starting_positions = vec![
        Point3d(-1, 0, 0),
        Point3d(2, 0, 0),
        Point3d(4, 0, 0),
        Point3d(3, 0, 0),
    ];

    let moons = starting_positions
        .into_iter()
        .map(|point| Moon::new_static_moon(point))
        .collect();

    moons
}

fn solve_day_12_1() -> () {
    let moons = create_moons();
    let mut simulator = MoonSimulator::new(moons);
    simulator.simulate_steps(1000);
    println!("Energy: {}", simulator.energy())
}

fn solve_day_12_2() {
    let moons = create_moons();
    let mut simulator = MoonSimulator::new(moons);
    println!("Period: {}", simulator.find_period().unwrap());
}

fn create_moons() -> Vec<Moon> {
    let starting_positions = vec![
        Point3d(17, 5, 1),
        Point3d(-2, -8, 8),
        Point3d(7, -6, 14),
        Point3d(1, -10, 4),
    ];
    let moons = starting_positions
        .into_iter()
        .map(|point| Moon::new_static_moon(point))
        .collect();
    moons
}
