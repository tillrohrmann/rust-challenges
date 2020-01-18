use aoc_2019_12::{Moon, MoonSimulator};
use aoc_common::math::Point3d;

fn main() {
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
    let mut simulator = MoonSimulator::new(moons);

    simulator.simulate_steps(1000);
    println!("Energy: {}", simulator.energy())
}
