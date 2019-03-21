#[macro_use] extern crate lazy_static;
use aoc_common::{GenericResult, GenericError};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Vector {
        Vector {
            x,
            y,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct PointWithVelocity {
    position: Vector,
    velocity: Vector,
}

impl PointWithVelocity {
    pub fn new(position: Vector, velocity: Vector) -> PointWithVelocity {
        PointWithVelocity {
            position,
            velocity,
        }
    }
}

pub fn parse_points_with_velocity(input: &Vec<String>) -> GenericResult<Vec<PointWithVelocity>> {
    input.iter().map(|input| parse_point_with_velocity(input)).collect()
}

fn parse_point_with_velocity(input: &str) -> GenericResult<PointWithVelocity> {
    lazy_static!{
        static ref regex: Regex = Regex::new(r"position=<(.*),(.*)> velocity=<(.*),(.*)>").unwrap();
    }

    regex
        .captures(input)
        .ok_or(GenericError::new("Could not parse point with velocity.").into())
        .and_then(|captures| {
            let x_pos = captures.get(1).ok_or(GenericError::new("Could not find x position.").into()).and_then(|m| parse_isize(m.as_str().trim()))?;
            let y_pos = captures.get(2).ok_or(GenericError::new("Could not find x position.").into()).and_then(|m| parse_isize(m.as_str().trim()))?;
            let x_velocity = captures.get(3).ok_or(GenericError::new("Could not find x position.").into()).and_then(|m| parse_isize(m.as_str().trim()))?;
            let y_velocity = captures.get(4).ok_or(GenericError::new("Could not find x position.").into()).and_then(|m| parse_isize(m.as_str().trim()))?;

            Ok(PointWithVelocity::new(
                Vector::new(x_pos, y_pos),
                Vector::new(x_velocity, y_velocity),
            ))
        })
}

fn parse_isize(input: &str) -> GenericResult<isize> {
    input.parse::<isize>().map_err(|e| e.into())
}

pub struct DisplayMap {
    points_with_velocity: Vec<PointWithVelocity>,
    min_vector: Vector,
    max_vector: Vector,
    field: Vec<Vec<char>>,
}

impl DisplayMap {
    const EMPTY_CHAR: char = ' ';
    const POINT_CHAR: char = '#';

    pub fn new(points_with_velocity: Vec<PointWithVelocity>) -> DisplayMap {
        let (min_x, max_x) = DisplayMap::find_min_max(&points_with_velocity, |pv| pv.position.x);
        let (min_y, max_y) = DisplayMap::find_min_max(&points_with_velocity, |pv| pv.position.y);
        let min_vector = Vector::new(min_x, min_y);
        let max_vector = Vector::new(max_x, max_y);

        let mut field = vec![vec![DisplayMap::EMPTY_CHAR; (max_vector.x - min_vector.x + 1) as usize]; (max_vector.y - min_vector.y + 1) as usize];

        for point_with_velocity in &points_with_velocity {
            let position = point_with_velocity.position;
            field[DisplayMap::normalize(position.y, min_vector.y)][DisplayMap::normalize(position.x, min_vector.x)] = DisplayMap::POINT_CHAR;
        }

        DisplayMap {
            points_with_velocity,
            min_vector,
            max_vector,
            field,
        }
    }

    fn normalize(coordinate: isize, min_coordinate: isize) -> usize {
        (coordinate - min_coordinate) as usize
    }

    fn normalize_x(&self, x: isize) -> usize {
        (x - self.min_vector.x) as usize
    }

    fn normalize_y(&self, y: isize) -> usize {
        (y - self.min_vector.y) as usize
    }

    fn find_min_max(points_with_velocity: &Vec<PointWithVelocity>, field_selector: fn(&PointWithVelocity) -> isize) -> (isize, isize) {
        let min = points_with_velocity.iter().map(field_selector).min().unwrap_or(0);
        let max = points_with_velocity.iter().map(field_selector).max().unwrap_or(0);

        (min, max)
    }

    pub fn display(&self) {
        for line in &self.field {
            println!("{:?}", line);
        }
    }

    pub fn advance(&mut self) {
        for point in self.points_with_velocity.iter() {
            let y = self.normalize_y(point.position.y);
            let x = self.normalize_x(point.position.x);
            self.field[y][x] = DisplayMap::EMPTY_CHAR;
        }

        for point in self.points_with_velocity.iter_mut() {
            point.position.x += point.velocity.x;
            point.position.y += point.velocity.y;
        }

        for point in self.points_with_velocity.iter() {
            let y = self.normalize_y(point.position.y);
            let x = self.normalize_x(point.position.x);
            self.field[y][x] = DisplayMap::POINT_CHAR;
        }
    }

    pub fn calculate_number_components(&self) -> usize {
        let mut components: Vec<Vec<usize>> = vec![vec![usize::max_value(); self.field[0].len()]; self.field.len()];
        let mut working_set = HashSet::new();

        for (index, point) in self.points_with_velocity.iter().enumerate() {
            let position = point.position;
            components[self.normalize_y(position.y)][self.normalize_x(position.x)] = index;
            working_set.insert(position);
        }

        while let Some(point) = working_set.iter().next() {
            let min_neighbour = find_min_neighbour(&components, point);
        }

        42
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let expected_point_with_velocity = PointWithVelocity::new(
            Vector::new(30432, -9912),
            Vector::new(-3, 1),
        );

        assert_eq!(parse_point_with_velocity("position=< 30432,  -9912> velocity=<-3,  1>").unwrap(), expected_point_with_velocity)
    }
}
