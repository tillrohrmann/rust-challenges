use aoc_common::math::Point;
use aoc_common::GenericResult;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

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

    fn vaporize_asteroids(&self, base: Point) -> AsteroidIterator {
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
        let (half_a, cos_angle_a, distance_a) = Map::calculate_asteroid_properties(a - base);
        let (half_b, cos_angle_b, distance_b) = Map::calculate_asteroid_properties(b - base);

        if half_a > half_b {
            Ordering::Less
        } else if half_a < half_b {
            Ordering::Greater
        } else {
            let sign = Map::get_sign(half_a) as f64;
            let cos_angle_a = cos_angle_a * sign;
            let cos_angle_b = cos_angle_b * sign;

            if cos_angle_a < cos_angle_b {
                Ordering::Less
            } else if cos_angle_a > cos_angle_b {
                Ordering::Greater
            } else {
                distance_a.partial_cmp(&distance_b).unwrap()
            }
        }
    }

    fn calculate_asteroid_properties(asteroid: Point) -> (i32, f64, f64) {
        let Point(x, y) = asteroid;
        let half = Map::get_sign(x as i32);
        let distance = asteroid.length();
        let cos_angle = y as f64 / distance;

        (half, cos_angle, distance)
    }

    fn get_sign(input: i32) -> i32 {
        if input >= 0 {
            1
        } else {
            -1
        }
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
.#####..#.#1####.###
##...#.####X#####...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let map = Map::parse_map(example_five);

        let mut asteroid_iterator = map.vaporize_asteroids(Point(11,13));

        let mut index = 0;
        while let Some(p) = asteroid_iterator.next() {
            index += 1;
            println!("{}: {:?}", index, p);
        }

        let mut asteroid_iterator = map.vaporize_asteroids(Point(11,13));

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
    }

    #[test]
    fn compare_asteroids() {
        assert_eq!(Map::compare_asteroids(Point(8, 3), Point(8, 1), Point(9, 2)), Ordering::Less);
    }
}
