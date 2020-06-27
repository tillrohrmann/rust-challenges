use aoc_common::GenericResult;
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

impl Into<char> for MapElement {
    fn into(self) -> char {
        unimplemented!()
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

    let width = lines[0].len();

    let result: Result<Vec<MapElement>, String> = lines.into_iter().flat_map(|line| {
        line.chars().map(|c| c.try_into()).collect::<Vec<Result<MapElement, String>>>()
    }).collect();

    Ok(Map::new(result?, width))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
