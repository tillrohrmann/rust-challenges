use aoc_common::math::Point;
use aoc_common::GenericResult;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

type FieldType = u8;

#[derive(PartialEq)]
struct Map {
    height: usize,
    width: usize,
    fields: Vec<FieldType>,
}

impl Map {
    fn new(height: usize, width: usize, fields: Vec<FieldType>) -> Map {
        Map {
            height,
            width,
            fields,
        }
    }

    fn parse(input: &Vec<String>) -> GenericResult<Map> {
        let numbers = input
            .into_iter()
            .flat_map(|line| {
                line.trim()
                    .split("")
                    .skip(1)
                    .take(line.len())
                    .map(|chr| FieldType::from_str(chr))
            })
            .collect::<Result<_, _>>()?;

        let height = input.len();
        let width = input[0].len();
        Ok(Map::new(height, width, numbers))
    }

    fn mut_iter(&mut self) -> MutMapIterator {
        MutMapIterator::new(self)
    }

    fn get_mut(&mut self, Point(x, y): Point) -> Option<&mut FieldType> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            self.fields.get_mut(x as usize + y as usize * self.width)
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.fields.chunks(self.width) {
            let line_string: String = line
                .iter()
                .map(|&entry| entry.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "{}", line_string)?;
        }

        std::fmt::Result::Ok(())
    }
}

struct MutMapIterator<'a> {
    index: usize,
    map: &'a mut Map,
}

impl<'a> MutMapIterator<'a> {
    fn new(map: &mut Map) -> MutMapIterator {
        MutMapIterator { index: 0, map }
    }
}

impl<'a> Iterator for MutMapIterator<'a> {
    type Item = (Point, &'a mut FieldType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.map.fields.len() {
            let reference = unsafe { &mut *(&mut self.map.fields[self.index] as *mut FieldType) };
            let result = Some((
                Point(
                    (self.index % self.map.width) as isize,
                    (self.index / self.map.width) as isize,
                ),
                reference,
            ));
            self.index += 1;

            result
        } else {
            None
        }
    }
}

pub struct Simulator {
    step: usize,
    num_flashes: usize,
    map: Map,
}

impl Display for Simulator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Step: {}, num flashes: {}", self.step, self.num_flashes)?;
        write!(f, "{}", self.map)?;

        std::fmt::Result::Ok(())
    }
}

impl Simulator {
    pub fn parse(input: &Vec<String>) -> GenericResult<Simulator> {
        let map = Map::parse(input)?;

        Ok(Simulator {
            step: 0,
            num_flashes: 0,
            map,
        })
    }

    pub fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.simulate_step();
        }
    }

    pub fn simulate_step(&mut self) {
        let mut flashed_points = HashSet::new();
        let mut flashing_points = Vec::new();

        for (address, value) in self.map.mut_iter() {
            *value += 1;
            if *value > 9 {
                flashing_points.push(address);
                flashed_points.insert(address);
            }
        }

        while let Some(point) = flashing_points.pop() {
            let neighbours = Simulator::neighbours(point);

            for neighbour in neighbours {
                if let Some(value) = self.map.get_mut(neighbour) {
                    *value += 1;

                    if *value > 9 && !flashed_points.contains(&neighbour) {
                        flashed_points.insert(neighbour);
                        flashing_points.push(neighbour);
                    }
                }
            }
        }

        for (_, value) in self.map.mut_iter() {
            if *value > 9 {
                *value = 0;
            }
        }

        self.num_flashes += flashed_points.len();
        self.step += 1;
    }

    fn neighbours(Point(xp, yp): Point) -> Vec<Point> {
        let result = vec![-1, 0, 1];

        result
            .iter()
            .flat_map(|&x| result.iter().map(move |&y| Point(x, y)))
            .collect::<Vec<Point>>()
            .into_iter()
            .filter(|point| *point != Point(0, 0))
            .map(|Point(x, y)| Point(x + xp, y + yp))
            .collect()
    }

    pub fn find_first_step_all_flash(&mut self) -> usize {
        loop {
            if self.all_flashing() {
                return self.step;
            } else {
                self.simulate_step();
            }
        }
    }

    pub fn get_num_flashes(&self) -> usize {
        self.num_flashes
    }

    fn all_flashing(&self) -> bool {
        self.map.fields.iter().all(|value| *value == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simulator() -> Simulator {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .split("\n")
            .map(|line| line.to_string())
            .collect();

        Simulator::parse(&input).unwrap()
    }

    fn create_map_after_single_step() -> Map {
        let input = "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

        create_map(input)
    }

    fn create_map(input: &str) -> Map {
        let input = input.split("\n").map(|line| line.to_string()).collect();

        Map::parse(&input).unwrap()
    }
    fn create_map_after_second_step() -> Map {
        let input = "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

        create_map(input)
    }

    #[test]
    fn simulate_single_step() {
        let mut simulator = create_simulator();
        simulator.simulate(1);
        assert_eq!(simulator.map, create_map_after_single_step());
    }

    #[test]
    fn simulate_two_steps() {
        let mut simulator = create_simulator();
        simulator.simulate(2);
        assert_eq!(simulator.map, create_map_after_second_step());
    }

    #[test]
    fn simulate_10_steps() {
        let mut simulator = create_simulator();
        simulator.simulate(10);
        assert_eq!(simulator.get_num_flashes(), 204);
    }
}
