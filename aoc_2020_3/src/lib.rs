use aoc_common::GenericResult;
use std::convert::{TryInto, TryFrom};
use std::fmt;
use std::fmt::Formatter;
use aoc_common::math::Point;

pub fn solve_slope(ride: Point, map: &Map) -> usize {
    let mut tree_counter = 0;
    let mut current_position = Point(0, 0);

    while (current_position.1 as usize) < map.height() {
        tree_counter += map.get(current_position).map(|element| {
            match element {
                MapElement::Tree => 1,
                _ => 0
            }
        }).unwrap_or(0);

        current_position = current_position + ride;
    }

    tree_counter
}

pub struct Map {
    map: Vec<Vec<MapElement>>,
}

impl Map {
    pub fn new(map: Vec<Vec<MapElement>>) -> Map {
        Map {
            map,
        }
    }

    pub fn get(&self, point: Point) -> Option<MapElement> {
        let Point(x, y) = point;
        let row = self.map.get(y as usize);

        row.and_then(|row| row.get((x as usize) % row.len()).copied())
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn parse_from_file(path: &str) -> GenericResult<Map> {
        let file_lines = aoc_common::read_raw_file_content(path)?;

        let parsed_lines = file_lines.into_iter().map(|line| Map::parse_line(line))
            .collect::<GenericResult<Vec<Vec<MapElement>>>>()?;

        Ok(Map::new(parsed_lines))
    }

    fn parse_line(line: String) -> GenericResult<Vec<MapElement>> {
        line.chars().into_iter().map(|char| {
            let result = char.try_into().map_err(|err: String| err.into());
            result
        }).collect()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.map.iter() {
            for column in row.iter() {
                write!(f, "{}", column);
            }
            writeln!(f, "");
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum MapElement {
    Tree,
    Open,
}

impl fmt::Display for MapElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapElement::Tree => write!(f, "#"),
            MapElement::Open => write!(f, "."),
        }?;

        Ok(())
    }
}

impl TryFrom<char> for MapElement {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapElement::Open),
            '#' => Ok(MapElement::Tree),
            _ => Err(format!("Could not parse character: {}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
