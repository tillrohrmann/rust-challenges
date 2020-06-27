use aoc_common::{GenericResult, GenericError};
use std::convert::{TryInto, TryFrom};
use core::{fmt, slice};
use std::fmt::Formatter;
use aoc_common::math::Point;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

pub struct Map {
    map: Vec<MapElement>,
    width: usize,
}

impl Map {
    fn new(map: Vec<MapElement>, width: usize) -> Map {
        Map{
            map,
            width,
        }
    }

    fn get(&self, point: Point) -> Option<MapElement> {
        let Point(x, y) = point;

        let index = x + y * self.width as isize;

        if index < 0 {
            None
        } else {
            self.map.get(index as usize).map(MapElement::to_owned)
        }
    }

    fn iter(&self) -> MapIterator<'_> {
        MapIterator::new(self.map.iter())
    }
}

struct MapIterator<'a> {
    vector_iterator: slice::Iter<'a, MapElement>,
}

impl<'a> MapIterator<'a> {
    fn new(vector_iterator: slice::Iter<'a, MapElement>) -> MapIterator<'a> {
        MapIterator {
            vector_iterator,
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a MapElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.vector_iterator.next()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, map_element) in self.map.iter().enumerate() {
            write!(f, "{}", map_element)?;

            if (idx+1) % self.width == 0 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
enum MapElement {
    Wall,
    Key(char),
    Door(char),
    Passage,
    Entrance,
}

impl TryFrom<char> for MapElement {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(MapElement::Wall),
            '.' => Ok(MapElement::Passage),
            '@' => Ok(MapElement::Entrance),
            x @ 'a'..='z' => Ok(MapElement::Key(x)),
            x @ 'A'..='Z' => Ok(MapElement::Door(x)),
            x => Err(format!("Unknown map element {}.", x))
        }
    }
}

impl fmt::Display for MapElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            MapElement::Wall => "#".to_string(),
            MapElement::Passage => ".".to_string(),
            MapElement::Entrance => "@".to_string(),
            MapElement::Key(key) => key.to_string(),
            MapElement::Door(door) => door.to_string(),
        };

        write!(f, "{}", result)
    }
}

pub fn read_map(filename: &str) -> GenericResult<Map> {
    let lines = aoc_common::read_raw_file_content(filename)?;

    read_map_from_lines(lines)
}

fn read_map_from_lines(lines: Vec<String>) -> GenericResult<Map> {
    let width = lines[0].len();
    let result: Result<Vec<MapElement>, String> = lines.into_iter().flat_map(|line| {
        line.chars().map(|c| c.try_into()).collect::<Vec<Result<MapElement, String>>>()
    }).collect();
    Ok(Map::new(result?, width))
}

fn read_from_string(map: &str) -> GenericResult<Map> {
    let lines: Vec<String> = map.split("\n").map(|l| l.to_string()).collect();

    read_map_from_lines(lines)
}

struct Solver<'a> {
    map: &'a Map,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Solver {
        Solver{
            map,
        }
    }

    fn try_solve(&self) -> Result<usize, String> {
        let key_set = Solver::find_all_keys(self.map);
        let starting_position = Solver::find(self.map, MapElement::Entrance).ok_or("No entrance found")?;

        let mut solution_candidates = BinaryHeap::new();

        solution_candidates.push(SolutionCandidate::new(starting_position, 0, HashSet::new(), Vec::new()));

        Ok(0)
    }

    fn solve(&self) -> usize {
        self.try_solve().unwrap()
    }

    fn find_all_keys(map: &Map) -> HashSet<char> {
        map.iter().flat_map(|element| {
            match *element {
                MapElement::Key(key) => Some(key),
                _ => None
            }
        }).collect()
    }

    fn find(map: &Map, map_element: MapElement) -> Option<Point> {
        None
    }
}

struct SolutionCandidate {
    position: Point,
    steps: usize,
    keys: HashSet<char>,
    history: Vec<char>,
}

impl SolutionCandidate {
    fn new(position: Point, steps: usize, keys: HashSet<char>, history: Vec<char>) -> SolutionCandidate {
        SolutionCandidate {
            position,
            steps,
            keys,
            history,
        }
    }

    fn collect_key_at(&self, new_position: Point, steps:usize, key: char) -> SolutionCandidate {
        let mut new_keys = HashSet::new();
        new_keys.extend(&self.keys);
        new_keys.insert(key);
        let mut new_history = self.history.clone();
        new_history.push(key);

        SolutionCandidate::new(new_position, self.steps + steps, new_keys, new_history)
    }
}

impl Ord for SolutionCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for SolutionCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.steps.partial_cmp(&other.steps)
    }
}

impl PartialEq for SolutionCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.steps.eq(&other.steps)
    }
}

impl Eq for SolutionCandidate {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_map() {
        let map = get_simple_map();
        run_test(&map, 8);
    }

    #[test]
    fn test_simple_map_2() {
        let map = get_simple_map_2();
        run_test(&map, 86);
    }

    #[test]
    fn test_simple_map_3() {
        let map = get_simple_map_3();
        run_test(&map, 132);
    }

    #[test]
    fn test_simple_map_4() {
        let map = get_simple_map_4();
        run_test(&map, 136);
    }

    #[test]
    fn test_simple_map_5() {
        let map = get_simple_map_5();
        run_test(&map, 81);
    }

    fn run_test(map: &Map, expected_result: usize) {
        let solver = Solver::new(map);

        assert_eq!(solver.solve(), expected_result);
    }

    fn get_simple_map() -> Map {
        let input = r"#########
#b.A.@.a#
#########";
        read_from_string(input).unwrap()
    }

    fn get_simple_map_2() -> Map {
        let input = r"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

        read_from_string(input).unwrap()
    }

    fn get_simple_map_3() -> Map {
        let input = r"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

        read_from_string(input).unwrap()
    }

    fn get_simple_map_4() -> Map {
        let input = r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        read_from_string(input).unwrap()
    }

    fn get_simple_map_5() -> Map {
        let input = r"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

        read_from_string(input).unwrap()
    }
}
