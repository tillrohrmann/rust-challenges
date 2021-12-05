use aoc_common::math::Point;
use aoc_common::GenericResult;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn draw(&self, map: &mut Map) {
        let diff = self.end - self.start;
        let step = if self.is_horizontal() {
            Point(diff.0 / diff.0.abs(), 0)
        } else if self.is_vertical() {
            Point(0, diff.1 / diff.1.abs())
        } else {
            Point(diff.0 / diff.0.abs(), diff.1 / diff.1.abs())
        };

        let mut point = self.start;

        while point != self.end {
            map.increment(point.0 as usize, point.1 as usize);
            point = point + step;
        }

        map.increment(self.end.0 as usize, self.end.1 as usize);
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("->");

        let start = splits.next().ok_or("No start point.".to_string())?;
        let end = splits.next().ok_or("No end point.".to_string())?;

        assert!(splits.next().is_none());

        let start =
            parse_point(start).map_err(|err| format!("Could not parse start point. {}", err))?;
        let end = parse_point(end).map_err(|err| format!("Could not parse end point. {}", err))?;

        Ok(Line { start, end })
    }
}

fn parse_point(input: &str) -> GenericResult<Point> {
    let mut splits = input.split(',');
    let x = splits.next().ok_or("No x value.")?.trim().parse()?;
    let y = splits.next().ok_or("No y value.")?.trim().parse()?;

    Ok(Point(x, y))
}

struct Lines {
    lines: Vec<Line>,
}

impl Lines {
    fn new(lines: Vec<Line>) -> Lines {
        Lines { lines }
    }

    fn parse_from_input(input: &[String]) -> GenericResult<Lines> {
        let lines = input
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Line>, String>>()?;

        Ok(Lines { lines })
    }

    fn draw(&self, map: &mut Map) {
        for line in &self.lines {
            line.draw(map);
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<u32>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.map.chunks(self.width) {
            write!(f, "{:?}\n", line)?;
        }

        Ok(())
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        let map = vec![0; width * height];
        Map { width, height, map }
    }

    fn increment(&mut self, x: usize, y: usize) {
        let index = x + y * self.width;
        self.map[index] = self.map[index] + 1
    }
}

pub fn calculate_overlapping_points_for_horizontal_vertical_lines(
    input: &Vec<String>,
) -> GenericResult<u32> {
    let lines = Lines::parse_from_input(input)?;
    let horizontal_vertical_lines = find_horizontal_vertical_lines(&lines);

    calculate_overlapping_points(&horizontal_vertical_lines)
}

fn calculate_overlapping_points(horizontal_vertical_lines: &Lines) -> Result<u32, Box<dyn Error>> {
    let (max_width, max_height) = find_max_width_height(&horizontal_vertical_lines);

    let mut map = Map::new(max_width + 1, max_height + 1);

    horizontal_vertical_lines.draw(&mut map);

    Ok(map.map.iter().filter(|&&value| value > 1).count() as u32)
}

pub fn calculate_overlapping_points_for_all_lines(input: &Vec<String>) -> GenericResult<u32> {
    let lines = Lines::parse_from_input(input)?;

    calculate_overlapping_points(&lines)
}

fn find_max_width_height(lines: &Lines) -> (usize, usize) {
    lines
        .lines
        .iter()
        .flat_map(|line| vec![line.start, line.end])
        .fold((0, 0), |(acc_x, acc_y), Point(x, y)| {
            (acc_x.max(x as usize), acc_y.max(y as usize))
        })
}

fn find_horizontal_vertical_lines(lines: &Lines) -> Lines {
    let horizontal_vertical_lines: Vec<Line> = lines
        .lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .cloned()
        .collect();

    Lines::new(horizontal_vertical_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_overlap() {
        let input = create_input();

        assert_eq!(
            calculate_overlapping_points_for_horizontal_vertical_lines(&input).unwrap(),
            5
        );
    }

    #[test]
    fn complex_overlap() {
        let input = create_input();

        assert_eq!(
            calculate_overlapping_points_for_all_lines(&input).unwrap(),
            12
        );
    }

    fn create_input() -> Vec<String> {
        vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ]
    }
}
