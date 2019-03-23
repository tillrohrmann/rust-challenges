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

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

pub struct PointMap {
    points_with_velocity: Vec<PointWithVelocity>,
    size: Vector,
}

impl PointMap {
    const EMPTY_CHAR: char = ' ';
    const POINT_CHAR: char = '#';

    pub fn new(points_with_velocity: Vec<PointWithVelocity>) -> PointMap {
        let (min_x, max_x) = PointMap::find_min_max(&points_with_velocity, |pv| pv.position.x);
        let (min_y, max_y) = PointMap::find_min_max(&points_with_velocity, |pv| pv.position.y);
        let min_vector = Vector::new(min_x, min_y);
        let size = Vector::new(max_x - min_x + 1, max_y - min_y + 1);

        let normalized_points_with_velocity = points_with_velocity.into_iter().map(|points_with_velocity| {
            PointWithVelocity::new(
                points_with_velocity.position - min_vector,
                points_with_velocity.velocity,
            )
        }).collect();

        PointMap {
            points_with_velocity: normalized_points_with_velocity,
            size,
        }
    }

    fn find_min_max(points_with_velocity: &Vec<PointWithVelocity>, field_selector: fn(&PointWithVelocity) -> isize) -> (isize, isize) {
        let min = points_with_velocity.iter().map(field_selector).min().unwrap_or(0);
        let max = points_with_velocity.iter().map(field_selector).max().unwrap_or(0);

        (min, max)
    }

    pub fn display(&self) {
        let mut field = vec![vec![PointMap::EMPTY_CHAR; self.size.x as usize]; self.size.y as usize];
        for point in self.points_with_velocity.iter() {
            field[point.position.y as usize][point.position.x as usize] = PointMap::POINT_CHAR;
        }

        for line in field.iter() {
            println!("{:?}", line);
        }
    }

    pub fn advance(&mut self) {
        for point in self.points_with_velocity.iter_mut() {
            point.position += point.velocity;
        }
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
