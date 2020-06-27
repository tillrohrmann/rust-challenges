use aoc_common::{GenericResult, GenericError};
use std::convert::{TryInto, TryFrom};
use core::fmt;
use std::fmt::Formatter;

pub struct Map {
    map: Vec<MapElement>,
    width: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, mapElement) in self.map.iter().enumerate() {
            write!(f, "{}", mapElement)?;

            if (idx+1) % self.width == 0 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

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

impl Map {
    fn new(map: Vec<MapElement>, width: usize) -> Map {
        Map{
            map,
            width,
        }
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

struct Solver {

}

impl Solver {
    fn new(map: &Map) -> Solver {
        Solver{}
    }

    fn solve(&self) -> usize {
        0
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
