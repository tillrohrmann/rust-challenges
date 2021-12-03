use aoc_common::{GenericResult, GenericError};
use std::convert::{TryInto, TryFrom};
use core::{fmt, slice, iter};
use std::fmt::Formatter;
use aoc_common::math::Point;
use std::collections::{HashSet, BinaryHeap, HashMap};
use std::cmp::{Ordering, Reverse};
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

impl MapElement {
    fn to_char(&self) -> char {
        match self {
            MapElement::Wall => '#',
            MapElement::Key(key) => *key,
            MapElement::Door(door) => *door,
            MapElement::Passage => '.',
            MapElement::Entrance => '@',
        }
    }
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

    fn solve(&self) -> usize {
        let graph = Graph::from_map(self.map);
        let number_keys = graph.nodes.iter().filter(|&node| node.is_key()).count();

        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();
        candidates.push(Reverse(SolutionCandidate::new(GraphElement::Entrance, 0, HashSet::new())));

        while let Some(Reverse(candidate)) = candidates.pop() {
            println!("{:?}", candidate);
            if candidate.keys.len() == number_keys {
                return candidate.steps;
            } else {
                if !visited.contains(&(candidate.position, candidate.get_sorted_keys())) {
                    visited.insert((candidate.position, candidate.get_sorted_keys()));
                    if let Some(neighbours) = graph.neighbour(&candidate.position) {
                        for neighbour in neighbours {
                            let can_go = match neighbour.node {
                                GraphElement::Entrance => true,
                                GraphElement::Door(door) => candidate.keys.contains(&door.to_ascii_lowercase()),
                                GraphElement::Key(key) => true
                            };

                            if can_go {
                                let new_key_set = match neighbour.node {
                                    GraphElement::Key(key) => {
                                        let mut new_key_set = candidate.keys.clone();
                                        new_key_set.insert(key);
                                        new_key_set
                                    },
                                    _ => candidate.keys.clone(),
                                };

                                candidates.push(Reverse(SolutionCandidate::new(neighbour.node, candidate.steps + neighbour.distance, new_key_set)));
                            }
                        }
                    }
                }
            }
        }

        0
    }
}

#[derive(PartialEq, Eq, Debug)]
struct SolutionCandidate {
    position: GraphElement,
    steps: usize,
    keys: HashSet<char>,
}

impl PartialOrd for SolutionCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.steps.partial_cmp(&other.steps)
    }
}

impl Ord for SolutionCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl SolutionCandidate {
    fn new(position: GraphElement, steps: usize, keys: HashSet<char>) -> SolutionCandidate {
        SolutionCandidate {
            position,
            steps,
            keys
        }
    }

    fn get_sorted_keys(&self) -> Vec<char> {
        let mut keys: Vec<char> = self.keys.iter().map(|&x| x).collect();
        keys.sort();

        keys
    }
}

pub fn solve_map(map: &Map) -> usize {
    let solver = Solver::new(map);

    solver.solve()
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum GraphElement {
    Key(char),
    Door(char),
    Entrance,
}

impl GraphElement {
    fn is_key(&self) -> bool {
        match self {
            GraphElement::Key(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&MapElement> for GraphElement {
    type Error = String;

    fn try_from(value: &MapElement) -> Result<Self, Self::Error> {
        match value {
            MapElement::Door(door) => Ok(GraphElement::Door(*door)),
            MapElement::Key(key) => Ok(GraphElement::Key(*key)),
            MapElement::Entrance => Ok(GraphElement::Entrance),
            _ => Err(format!("Cannot convert {} into a GraphElement.", value))
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<GraphElement>,
    neighbours: HashMap<GraphElement, HashSet<Neighbour>>,
}

impl Graph {
    fn from_map(map: &Map) -> Graph {
        let neighbours: Result<HashMap<GraphElement, HashSet<Neighbour>>, String> = map.pos_iter()
            .filter(|(_, map_element)| Graph::is_node_candidate(map_element))
            .map(|(starting_point, map_element)| (map_element.try_into().map(|m| (m, Graph::find_neighbours(starting_point, map)))))
            .collect();

        let neighbours = neighbours.unwrap();

        Graph {
            nodes: neighbours.keys().map(|&x| x).collect(),
            neighbours: neighbours,
        }
    }

    fn is_node_candidate(map_element: &MapElement) -> bool {
        match map_element {
            MapElement::Door(_) | MapElement::Key(_) | MapElement::Entrance => true,
            _ => false
        }
    }

    fn find_neighbours(starting_point: Point, map: &Map) -> HashSet<Neighbour> {
        let directions = vec![Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)];
        let mut search_front = Vec::new();
        let mut visited = HashSet::new();
        let mut result = HashSet::new();

        search_front.push(Candidate::new(starting_point, 0));
        visited.insert(starting_point);

        while let Some(next_candidate) = search_front.pop() {
            if let Some(map_element) = map.get(next_candidate.new_position) {
                if Graph::is_node_candidate(&map_element) && next_candidate.new_position != starting_point {
                    result.insert(Neighbour::new((&map_element).try_into().unwrap(), next_candidate.steps));
                } else {
                    let next_steps: Vec<Point> = directions.iter().map(|&direction| next_candidate.new_position + direction).collect();

                    let next_steps: Vec<Point> = next_steps.into_iter()
                        .filter(|next_step|
                            !visited.contains(next_step) &&
                                map
                                    .get(*next_step)
                                    .map_or(false, |element| element != MapElement::Wall))
                        .collect();

                    for next_step in next_steps {
                        visited.insert(next_step);
                        search_front.push(Candidate::new(next_step, next_candidate.steps + 1))
                    }
                }
            }
        }

        result
    }

    fn neighbour(&self, graph_element: &GraphElement) -> Option<&HashSet<Neighbour>> {
        self.neighbours.get(graph_element)
    }
}

struct Candidate {
    new_position: Point,
    steps: usize,
}

impl Candidate {
    fn new(position: Point, steps: usize) -> Candidate {
        Candidate {
            new_position: position,
            steps,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Neighbour {
    node: GraphElement,
    distance: usize,
}

impl Neighbour {
    fn new(node: GraphElement, distance: usize) -> Neighbour {
        Neighbour{
            node,
            distance,
        }
    }
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
