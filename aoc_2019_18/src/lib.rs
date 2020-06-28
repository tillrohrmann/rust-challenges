use aoc_common::{GenericResult, GenericError};
use std::convert::{TryInto, TryFrom};
use core::{fmt, slice, iter};
use std::fmt::Formatter;
use aoc_common::math::Point;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::iter::Enumerate;

pub struct Map {
    map: Vec<MapElement>,
    width: usize,
}

impl Map {
    fn new(map: Vec<MapElement>, width: usize) -> Map {
        Map {
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

    fn pos_iter(&self) -> MapPosIterator<'_> {
        MapPosIterator::new(self.map.iter().enumerate(), self.width)
    }
}

struct MapPosIterator<'a> {
    map_pos_iterator: Enumerate<slice::Iter<'a, MapElement>>,
    width: usize,
}

impl<'a> MapPosIterator<'a> {
    fn new(map_pos_iterator: Enumerate<slice::Iter<'a, MapElement>>, width: usize) -> MapPosIterator<'a> {
        MapPosIterator {
            map_pos_iterator,
            width,
        }
    }
}

impl<'a> Iterator for MapPosIterator<'a> {
    type Item = (Point, &'a MapElement);

    fn next(&mut self) -> Option<Self::Item> {
        self.map_pos_iterator.next().map(|(idx, map_element)| (Point((idx % self.width) as isize, (idx / self.width) as isize), map_element))
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

            if (idx + 1) % self.width == 0 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
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
        Solver {
            map,
        }
    }

    fn try_solve(&self) -> Result<usize, String> {
        let key_set = Solver::find_all_keys(self.map);
        let starting_position = Solver::find(self.map, MapElement::Entrance).ok_or("No entrance found")?;

        let mut solution_candidates = BinaryHeap::new();

        solution_candidates.push(SolutionCandidate::new(starting_position, 0, HashSet::new(), Vec::new()));

        while let Some(current_solution_candidate) = solution_candidates.pop() {
            if key_set == *current_solution_candidate.get_keys() {
                return Ok(current_solution_candidate.get_steps())
            }

            let paths: Vec<PathToKey> = Solver::find_paths_to_keys(current_solution_candidate.get_position(), current_solution_candidate.get_keys(), self.map);

            for path_to_key in paths {
                let new_solution_candidate = current_solution_candidate.collect_key_at(path_to_key.position, path_to_key.steps, path_to_key.key);
                solution_candidates.push(new_solution_candidate);
            }
        }

        Err("Could not find a valid solution".to_string())
    }

    fn find_paths_to_keys(position: Point, keys: &HashSet<char>, map: &Map) -> Vec<PathToKey> {
        let mut positions = Vec::new();
        positions.push((position, 0));
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        while let Some((current_position, steps)) = positions.pop() {
            let element = map.get(current_position).unwrap();

            if let MapElement::Key(key) = element {
                result.push(PathToKey::new(key, steps, current_position))
            } else {
                let neighbours: Vec<Point> = Solver::find_neighbours(current_position, map, keys);

                for neighbour in neighbours {
                    if !visited.contains(&neighbour) {
                        visited.insert(neighbour);
                        positions.push((neighbour, steps + 1))
                    }
                }
            }
        }

        result
    }

    fn find_neighbours(position: Point, map: &Map, keys: &HashSet<char>) -> Vec<Point> {
        vec![]
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

    fn find(map: &Map, search_element: MapElement) -> Option<Point> {
        map.pos_iter().find(|&(_, map_element)| search_element == *map_element).map(|(p, _)| p)
    }
}

struct PathToKey {
    key: char,
    steps: usize,
    position: Point,
}

impl PathToKey {
    fn new(key: char, steps: usize, position: Point) -> PathToKey {
        PathToKey {
            key,
            steps,
            position,
        }
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

    fn get_position(&self) -> Point {
        self.position
    }

    fn get_keys(&self) -> &HashSet<char> {
        &self.keys
    }

    fn get_steps(&self) -> usize {
        self.steps
    }

    fn collect_key_at(&self, new_position: Point, steps: usize, key: char) -> SolutionCandidate {
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

impl Eq for SolutionCandidate {}

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
