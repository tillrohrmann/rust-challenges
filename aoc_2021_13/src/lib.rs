#[macro_use]
extern crate lazy_static;

use regex::Regex;

use aoc_common::math::Point;
use aoc_common::GenericResult;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Paper {
    points: HashSet<Point>,
}

impl Paper {
    pub fn new(points: HashSet<Point>) -> Paper {
        Paper { points }
    }

    pub fn fold(&self, instruction: &FoldingInstruction) -> Paper {
        let new_points = self
            .points
            .iter()
            .flat_map(|point| instruction.fold(point))
            .collect();

        Paper::new(new_points)
    }

    pub fn count_points(&self) -> usize {
        self.points.len()
    }

    fn parse(input: &[String]) -> GenericResult<Paper> {
        let points = input
            .iter()
            .map(|line| parse_point(line))
            .collect::<GenericResult<_>>()?;

        Ok(Paper::new(points))
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self
            .points
            .iter()
            .fold((0, 0), |(max_x, max_y), &Point(x, y)| {
                (x.max(max_x), y.max(max_y))
            });

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.points.contains(&Point(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn parse_point(line: &String) -> GenericResult<Point> {
    let mut splits = line.split(',');

    let x = splits
        .next()
        .ok_or("Point needs to have the format x,y")?
        .parse()?;

    let y = splits
        .next()
        .ok_or("Point needs to have the format x,y")?
        .parse()?;

    Ok(Point(x, y))
}

#[derive(Copy, Clone)]
pub struct FoldingInstruction {
    value: isize,
    axis: Axis,
}

impl FoldingInstruction {
    fn new(axis: Axis, value: isize) -> FoldingInstruction {
        FoldingInstruction { axis, value }
    }

    fn fold(&self, &Point(x, y): &Point) -> Option<Point> {
        match self.axis {
            Axis::X => {
                if x < self.value {
                    Some(Point(x, y))
                } else if x > self.value {
                    Some(Point(2 * self.value - x, y))
                } else {
                    None
                }
            }
            Axis::Y => {
                if y < self.value {
                    Some(Point(x, y))
                } else if y > self.value {
                    Some(Point(x, 2 * self.value - y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

impl FromStr for Axis {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(format!("Unknown pattern {}.", s)),
        }
    }
}

pub fn parse_input(input: &Vec<String>) -> GenericResult<(Paper, Vec<FoldingInstruction>)> {
    let empty_line_index = input
        .iter()
        .enumerate()
        .find(|(idx, line)| line.is_empty())
        .map(|(idx, _)| idx)
        .ok_or("Could not find empty line.")?;

    let (paper_input, folding_instructions_input) = input.split_at(empty_line_index);

    let paper = Paper::parse(paper_input)?;
    let folding_instructions = parse_folding_instructions(&folding_instructions_input[1..])?;

    Ok((paper, folding_instructions))
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
}

fn parse_folding_instructions(input: &[String]) -> GenericResult<Vec<FoldingInstruction>> {
    input
        .iter()
        .map(|line| parse_folding_instruction(line))
        .collect()
}

fn parse_folding_instruction(instruction: &String) -> GenericResult<FoldingInstruction> {
    if let Some(captures) = RE.captures(instruction) {
        let axis = captures.get(1).unwrap().as_str().parse()?;
        let value = captures.get(2).unwrap().as_str().parse()?;

        Ok(FoldingInstruction::new(axis, value))
    } else {
        Err(format!("Instruction did not follow syntax: {}", instruction).into())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
