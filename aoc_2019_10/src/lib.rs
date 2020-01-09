use aoc_common::math::Point;
use aoc_common::GenericResult;
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
            .filter(|&&asteroid| {
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
            })
            .count()
            == 0
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
        let input = "\
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

        let map = Map::parse_map(input);
        assert_eq!(map.find_best_asteroid(), Some((Point(11, 13), 210)))
    }
}
