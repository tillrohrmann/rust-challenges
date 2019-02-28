use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt;
use std::collections::VecDeque;

type GenericResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct GenericError {
    message: String,
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError {
            message: message.to_string(),
        }
    }
}

impl Error for GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GenericError: {}", self.message)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Coordinate(usize, usize);

#[derive(Clone, PartialEq, Debug)]
enum Field {
    Free,
    Taken(u8, usize),
    Border,
}

#[derive(PartialEq, Debug)]
struct Map {
    height: usize,
    width: usize,
    map: Vec<Field>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            height,
            width,
            map: vec![Field::Free; height * width],
        }
    }

    fn flatten_coords(x: usize, y: usize, width: usize) -> usize {
        x + y * width
    }

    pub fn get(&self, x: usize, y: usize) -> &Field {
        &self.map[Map::flatten_coords(x, y, self.width)]
    }

    pub fn set(&mut self, x: usize, y: usize, field: Field) {
        self.map[Map::flatten_coords(x, y, self.width)] = field;
    }
}

fn read_file_content(path: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let buf_reader = BufReader::new(&file);

    buf_reader.lines().collect()
}

fn parse_coordinates(input: &str) -> GenericResult<Coordinate> {
    let splits: Vec<&str> = input.trim().split(',').collect();

    if splits.len() == 2 {
        let result = splits.iter().map(|&element| element.trim().parse::<usize>()).collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(Coordinate(result[0], result[1]))
    } else {
        Err(GenericError::new(&format!("Invalid line: {}", input)).into())
    }
}

struct Candidate(u8, usize, Coordinate);

impl Candidate {
    fn next_candidates(&self, width: usize, height: usize) -> Vec<Candidate> {
        let Candidate(self_id, distance, Coordinate(ux, uy)) = *self;
        let x = ux as isize;
        let y = uy as isize;
        let new_distance = distance + 1;
        let result: Vec<(isize, isize)> = vec![
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
            (x, y - 1)];
        result.into_iter()
            .filter(|&(x, y)| 0 <= x && x < width as isize && 0 <= y && y < height as isize)
            .map(|(x, y)| Candidate(self_id, new_distance, Coordinate(x as usize, y as usize)))
            .collect()
    }
}

fn create_map(initial_points: &Vec<Coordinate>) -> Map {
    let (max_width, max_height) = initial_points.iter().fold((0, 0), |(width, height), &Coordinate(x, y)| (usize::max(width, x), usize::max(height, y)));
    let mut map = Map::new(max_width + 1, max_height + 1);

    let mut queue: VecDeque<Candidate> = initial_points.iter().enumerate().map(|(index, coordinate)| Candidate(index as u8, 0,coordinate.clone())).collect();

    while let Some(candidate) = queue.pop_front() {
        let Candidate(id, distance, Coordinate(x, y)) = candidate;
        let add_new_candidates = match map.get(x, y) {
            Field::Free => {
                map.set(x, y, Field::Taken(id, distance));
                true
            },
            &Field::Taken(other_id, other_distance) => {
                if other_id != id && other_distance == distance {
                    map.set(x, y, Field::Border);
                    true
                } else {
                    false
                }
            },
            _ => false
        };

        if add_new_candidates {
            let next_candidates: Vec<Candidate> = candidate.next_candidates(map.width, map.height);
            for next_candidate in next_candidates.into_iter() {
                queue.push_back(next_candidate)
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinates() {
        assert_eq!(parse_coordinates("1, 1").unwrap(), Coordinate(1, 1));
    }

    #[test]
    fn test_create_map() {
        let mut map = Map::new(3, 3);
        map.set(0, 0, Field::Taken(0, 0));
        map.set(0, 1, Field::Taken(0, 1));
        map.set(0, 2, Field::Border);
        map.set(1, 0, Field::Taken(0, 1));
        map.set(1, 1, Field::Border);
        map.set(1, 2, Field::Taken(1, 1));
        map.set(2,0, Field::Border);
        map.set(2, 1, Field::Taken(1, 1));
        map.set(2, 2, Field::Taken(1, 0));
        assert_eq!(create_map(&vec![Coordinate(0, 0), Coordinate(2, 2)]), map);
    }
}
