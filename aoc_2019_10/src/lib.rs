use aoc_common::math::Point;
use aoc_common::GenericResult;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use std::ops::{Div, Mul};

pub struct Map {
    asteroids: HashSet<Point>,
}

impl Map {
    pub fn load_from_file(filename: &str) -> GenericResult<Map> {
        let raw_content = fs::read_to_string(filename)?;
        Ok(Map::parse_map(&raw_content))
    }

    pub fn parse_map(input: &str) -> Map {
        let asteroids = input
            .split("\n")
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .flat_map(move |(column, chr)| {
                        if chr == '#' {
                            Some(Point(column as isize, row as isize))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        Map { asteroids }
    }

    pub fn find_best_asteroid(&self) -> Option<(Point, u32)> {
        self.asteroids
            .iter()
            .map(|&asteroid| (asteroid, self.number_visible_asteroids_from(asteroid)))
            .max_by_key(|(_, number)| *number)
    }

    fn number_visible_asteroids_from(&self, asteroid: Point) -> u32 {
        self.asteroids
            .iter()
            .filter(|&&other| asteroid != other && self.is_visible_from(asteroid, other))
            .count() as u32
    }

    fn is_visible_from(&self, asteroid_a: Point, asteroid_b: Point) -> bool {
        self.asteroids
            .iter()
            .filter(|&&asteroid| asteroid_a != asteroid && asteroid_b != asteroid)
            .filter(|&&asteroid| Map::is_in_between(asteroid_a, asteroid_b, asteroid))
            .count()
            == 0
    }

    fn is_in_between(asteroid_a: Point, asteroid_b: Point, asteroid: Point) -> bool {
        let a_b = asteroid_b - asteroid_a;
        let a_asteroid = asteroid - asteroid_a;

        if a_b.0 * a_asteroid.1 == a_b.1 * a_asteroid.0 {
            let t = match a_b {
                Point(0, 0) => panic!("Asteroid a and b should not be the same."),
                Point(x, 0) => a_asteroid.0 as f64 / x as f64,
                Point(_, y) => a_asteroid.1 as f64 / y as f64,
            };

            0.0 < t && t < 1.0
        } else {
            false
        }
    }

    pub fn vaporize_asteroids(&self, base: Point) -> AsteroidIterator {
        let mut translated_asteroids: Vec<Point> = self
            .asteroids
            .iter()
            .filter(|&&other| other != base)
            .map(|&x| x)
            .collect();

        translated_asteroids.sort_by(|&a, &b| Map::compare_asteroids(base, a, b));

        AsteroidIterator::new(base,translated_asteroids)
    }

    fn compare_asteroids(base: Point, a: Point, b: Point) -> Ordering {
        let (quarter_a, y_a, distance_sqrd_a) = Map::calculate_asteroid_properties(a - base);
        let (quarter_b, y_b, distance_sqrd_b) = Map::calculate_asteroid_properties(b - base);

        if quarter_a > quarter_b {
            Ordering::Greater
        } else if quarter_a < quarter_b {
            Ordering::Less
        } else {
            let sign = quarter_a.get_sign();
            let y_a = y_a;
            let y_b = y_b;
            let cos_a_times_distance_sqrd_b = sign as isize * y_a * y_a * distance_sqrd_b;
            let cos_b_times_distance_sqrd_a = sign as isize * y_b * y_b * distance_sqrd_a;

            if cos_a_times_distance_sqrd_b < cos_b_times_distance_sqrd_a {
                Ordering::Less
            } else if cos_a_times_distance_sqrd_b > cos_b_times_distance_sqrd_a {
                Ordering::Greater
            } else {
                distance_sqrd_a.partial_cmp(&distance_sqrd_b).unwrap()
            }
        }
    }

    fn calculate_asteroid_properties(asteroid: Point) -> (Quarter, isize, isize) {
        let Point(x, y) = asteroid;
        let quarter = Quarter::from_point(asteroid);
        let distance_sqrd = asteroid.length_sqrd();

        (quarter, y, distance_sqrd)
    }
}

#[derive(Debug)]
enum Quarter {
    First,
    Second,
    Third,
    Fourth,
}

impl Quarter {
    fn from_point(point: Point) -> Quarter {
        let Point(x, y) = point;

        if x >= 0 && y < 0 {
            Quarter::First
        } else if x >= 0 && y >= 0 {
            Quarter::Second
        } else if x < 0 && y >= 0 {
            Quarter::Third
        } else {
            Quarter::Fourth
        }
    }

    fn get_sign(&self) -> i32 {
        match self {
            Quarter::First => -1,
            Quarter::Second => 1,
            Quarter::Third => -1,
            Quarter::Fourth => 1,
        }
    }

    fn get_numeric_value(&self) -> i32 {
        match self {
            Quarter::First => 1,
            Quarter::Second => 2,
            Quarter::Third => 3,
            Quarter::Fourth => 4,
        }
    }
}

impl PartialOrd for Quarter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_numeric_value().partial_cmp(&other.get_numeric_value())
    }
}

impl PartialEq for Quarter {
    fn eq(&self, other: &Self) -> bool {
        self.get_numeric_value().eq(&other.get_numeric_value())
    }
}

pub struct AsteroidIterator {
    sorted_asteroids: Vec<Option<Point>>,
    last_returned_asteroid: Option<Point>,
    base: Point,
    current_idx: usize,
    returned_values: usize,
}

impl AsteroidIterator {
    fn new(base: Point, sorted_asteroids: Vec<Point>) -> AsteroidIterator {
        let sorted_asteroids = sorted_asteroids.into_iter().map(|x| Some(x)).collect();
        AsteroidIterator {
            sorted_asteroids,
            last_returned_asteroid: None,
            base,
            current_idx: 0,
            returned_values: 0,
        }
    }
}

impl Iterator for AsteroidIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.returned_values >= self.sorted_asteroids.len() {
            None
        } else {
            loop {
                if let Some(new_value) = self.sorted_asteroids[self.current_idx] {
                    let return_value = if let Some(previous_value) = self.last_returned_asteroid {
                        !Map::is_in_between(self.base, new_value, previous_value)
                    } else {
                        true
                    };

                    if return_value {
                        std::mem::replace(&mut self.sorted_asteroids[self.current_idx], None);
                        self.last_returned_asteroid = Some(new_value);
                        self.returned_values += 1;
                        return Some(new_value);
                    }
                }

                self.current_idx += 1;

                if self.current_idx >= self.sorted_asteroids.len() {
                    self.current_idx = 0;
                    self.last_returned_asteroid = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_asteroid_example_one() {
        let input = "\
.#..#
.....
#####
....#
...##";

        let map = Map::parse_map(input);
        assert_eq!(map.is_visible_from(Point(3, 4), Point(1, 0)), false);
        assert_eq!(map.find_best_asteroid(), Some((Point(3, 4), 8)));
    }

    #[test]
    fn best_asteroids_example_two() {
        let input = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let map = Map::parse_map(input);
        assert_eq!(map.find_best_asteroid(), Some((Point(5, 8), 33)))
    }

    #[test]
    fn best_asteroids_example_three() {
        let input = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let map = Map::parse_map(input);
        assert_eq!(map.find_best_asteroid(), Some((Point(1, 2), 35)))
    }

    #[test]
    fn best_asteroids_example_four() {
        let input = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let map = Map::parse_map(input);
        assert_eq!(map.find_best_asteroid(), Some((Point(6, 3), 41)))
    }

    #[test]
    fn best_asteroids_example_five() {
        let example_five = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let map = Map::parse_map(example_five);
        assert_eq!(map.find_best_asteroid(), Some((Point(11, 13), 210)))
    }

    #[test]
    fn vaporization_order() {
        let input = "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

        let map = Map::parse_map(input);

        assert_eq!(
            map.vaporize_asteroids(Point(8, 3)).collect::<Vec<Point>>(),
            vec![
                Point(8, 1),
                Point(9, 0),
                Point(9, 1),
                Point(10, 0),
                Point(9, 2),
                Point(11, 1),
                Point(12, 1),
                Point(11, 2),
                Point(15, 1),
                Point(12, 2),
                Point(13, 2),
                Point(14, 2),
                Point(15, 2),
                Point(12, 3),
                Point(16, 4),
                Point(15, 4),
                Point(10, 4),
                Point(4, 4),
                Point(2, 4),
                Point(2, 3),
                Point(0, 2),
                Point(1, 2),
                Point(0, 1),
                Point(1, 1),
                Point(5, 2),
                Point(1, 0),
                Point(5, 1),
                Point(6, 1),
                Point(6, 0),
                Point(7, 0),
                Point(8, 0),
                Point(10, 1),
                Point(14, 0),
                Point(16, 1),
                Point(13, 3),
                Point(14, 3),
            ]
        );
    }

    #[test]
    fn vaporization_order_example_five() {
        let example_five = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let map = Map::parse_map(example_five);

        let base = Point(11, 13);
        let mut asteroid_iterator = map.vaporize_asteroids(base);

        let mut asteroid_iterator = map.vaporize_asteroids(base);

        assert_eq!(asteroid_iterator.next(), Some(Point(11, 12)));
        assert_eq!(asteroid_iterator.next(), Some(Point(12, 1)));
        assert_eq!(asteroid_iterator.next(), Some(Point(12, 2)));
        let mut asteroid_iterator= asteroid_iterator.skip(6);
        assert_eq!(asteroid_iterator.next(), Some(Point(12, 8))); // 10th
        let mut asteroid_iterator= asteroid_iterator.skip(9);
        assert_eq!(asteroid_iterator.next(), Some(Point(16, 0))); // 20th
        let mut asteroid_iterator= asteroid_iterator.skip(29);
        assert_eq!(asteroid_iterator.next(), Some(Point(16, 9))); // 50th
        let mut asteroid_iterator= asteroid_iterator.skip(49);
        assert_eq!(asteroid_iterator.next(), Some(Point(10, 16))); // 100th
        let mut asteroid_iterator = asteroid_iterator.skip(98);
        assert_eq!(asteroid_iterator.next(), Some(Point(9, 6))); // 199th
        assert_eq!(asteroid_iterator.next(), Some(Point(8, 2))); // 200th
        assert_eq!(asteroid_iterator.next(), Some(Point(10, 9))); // 201st
        let mut asteroid_iterator = asteroid_iterator.skip(97);
        assert_eq!(asteroid_iterator.next(), Some(Point(11, 1)));
        assert_eq!(asteroid_iterator.next(), None);
    }

    #[test]
    fn compare_asteroids() {
        assert_eq!(Map::compare_asteroids(Point(8, 3), Point(8, 1), Point(9, 2)), Ordering::Less);
    }
}
