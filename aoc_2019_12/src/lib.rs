use aoc_common::math::Point3d;
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Moon {
    position: Point3d,
    velocity: Point3d,
}

impl Moon {
    fn new(position: Point3d, velocity: Point3d) -> Moon {
        Moon { position, velocity }
    }

    pub fn new_static_moon(position: Point3d) -> Moon {
        Moon::new(position, Point3d(0, 0, 0))
    }

    fn energy(&self) -> usize {
        let Point3d(x, y, z) = self.position;
        let Point3d(r, s, t) = self.velocity;

        let potential = Moon::calculate_energy(vec![x, y, z]);
        let kinetic = Moon::calculate_energy(vec![r, s ,t]);

        kinetic * potential
    }

    fn calculate_energy(values: Vec<isize>) -> usize {
        values.into_iter().map(|x| x.abs() as usize).sum()
    }

    fn update_velocity(&mut self, gravity: Point3d) {
        self.velocity = self.velocity + gravity;
    }

    fn update_position(&mut self) {
        self.position = self.position + self.velocity;
    }
}

#[derive(Debug)]
pub struct MoonSimulator {
    moons: Vec<Moon>,
}

impl MoonSimulator {
    pub fn new(moons: Vec<Moon>) -> MoonSimulator {
        MoonSimulator { moons }
    }

    pub fn energy(&self) -> usize {
        self.moons.iter().map(|moon| moon.energy()).sum()
    }

    pub fn simulate_steps(&mut self, steps: isize) {
        for _ in 0..steps {
            self.simulate_step();
        }
    }

    fn simulate_step(&mut self) {
        self.update_velocity();
        self.moons
            .iter_mut()
            .for_each(|moon| moon.update_position())
    }

    fn update_velocity(&mut self) -> () {
        for i in 0..self.moons.len() - 1 {
            for j in i + 1..self.moons.len() {
                let (gravity_a, gravity_b) =
                    MoonSimulator::calculate_gravity(&self.moons[i], &self.moons[j]);
                self.moons[i].update_velocity(gravity_a);
                self.moons[j].update_velocity(gravity_b);
            }
        }
    }

    fn calculate_gravity(moon_a: &Moon, moon_b: &Moon) -> (Point3d, Point3d) {
        let Point3d(x_a, y_a, z_a) = moon_a.position;
        let Point3d(x_b, y_b, z_b) = moon_b.position;

        let gravity = Point3d(
            MoonSimulator::gravity(x_a, x_b),
            MoonSimulator::gravity(y_a, y_b),
            MoonSimulator::gravity(z_a, z_b),
        );

        (gravity, gravity * -1)
    }

    fn gravity(a: isize, b: isize) -> isize {
        match a.cmp(&b) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_of_moon() {
        let moon = Moon::new(Point3d(1, 1, 1), Point3d(-1, 2, -1));

        assert_eq!(moon.energy(), 12)
    }

    fn create_moons_example_one() -> Vec<Moon> {
        let positions = vec![
            Point3d(-1, 0, 2),
            Point3d(2, -10, -7),
            Point3d(4, -8, 8),
            Point3d(3, 5, -1),
        ];

        positions
            .into_iter()
            .map(|position| Moon::new_static_moon(position))
            .collect()
    }

    #[test]
    fn example_one_step_1() {
        let expected_moon_states = vec![
            Moon::new(Point3d(2, -1, 1), Point3d(3, -1, -1)),
            Moon::new(Point3d(3, -7, -4), Point3d(1, 3, 3)),
            Moon::new(Point3d(1, -7, 5), Point3d(-3, 1, -3)),
            Moon::new(Point3d(2, 2, 0), Point3d(-1, -3, 1)),
        ];
        let mut simulator = MoonSimulator::new(create_moons_example_one());
        run_moon_simulator_test(&mut simulator, expected_moon_states, 1)
    }

    #[test]
    fn example_one_step_10() {
        let expected_moon_states = vec![
            Moon::new(Point3d(2, 1, -3), Point3d(-3, -2, 1)),
            Moon::new(Point3d(1, -8, 0), Point3d(-1, 1, 3)),
            Moon::new(Point3d(3, -6, 1), Point3d(3, 2, -3)),
            Moon::new(Point3d(2, 0, 4), Point3d(1, -1, -1)),
        ];
        let mut simulator = MoonSimulator::new(create_moons_example_one());
        run_moon_simulator_test(&mut simulator, expected_moon_states, 10);
        assert_eq!(simulator.energy(), 179);
    }

    fn run_moon_simulator_test(
        simulator: &mut MoonSimulator,
        expected_moon_states: Vec<Moon>,
        i: isize,
    ) -> () {
        simulator.simulate_steps(i);
        assert_eq!(simulator.moons, expected_moon_states)
    }
}
