use crate::LineDirections::Left;
use hamcrest2::prelude::*;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug)]
enum LineDirections {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

impl LineDirections {
    fn to_relative_point(&self) -> Point {
        match *self {
            LineDirections::Right(right) => Point::new(right, 0),
            LineDirections::Left(left) => Point::new(-left, 0),
            LineDirections::Up(up) => Point::new(0, up),
            LineDirections::Down(down) => Point::new(0, -down),
        }
    }
}

struct LineSegments {
    line_segments: Vec<Line>,
}

impl LineSegments {
    pub fn from_line_directions(line_directions: Vec<LineDirections>) -> LineSegments {
        let mut line_segments = Vec::new();
        let mut start_point = Point::new(0, 0);

        for line_direction in &line_directions {
            let end_point = start_point.plus(&line_direction.to_relative_point());
            line_segments.push(Line::new(start_point, end_point));
            start_point = end_point;
        }

        LineSegments { line_segments }
    }
}

impl IntoIterator for LineSegments {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.line_segments.into_iter()
    }
}

impl<'a> IntoIterator for &'a LineSegments {
    type Item = &'a Line;
    type IntoIter = std::slice::Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.line_segments).into_iter()
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn calculate_intersections(&self, other: &Line) -> Vec<Point> {
        let min_self = self.start.min(&self.end);
        let max_self = self.start.max(&self.end);

        let min_other = other.start.min(&other.end);
        let max_other = other.start.max(&other.end);

        let start_overlap = min_self.max(&min_other);
        let end_overlap = max_self.min(&max_other);

        if start_overlap.x == end_overlap.x || start_overlap.y == end_overlap.y {
            let mut result = Vec::new();
            for x in start_overlap.x..=end_overlap.x {
                for y in start_overlap.y..=end_overlap.y {
                    result.push(Point::new(x, y))
                }
            }

            result
        } else {
            vec![]
        }
    }

    fn direction(&self) -> Point {
        self.end.minus(&self.start)
    }

    fn length(&self) -> u32 {
        self.end.minus(&self.start).manhattan_distance()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn plus(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn minus(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn dot_product(&self, other: &Point) -> i32 {
        self.x * other.x + self.y * other.y
    }

    fn min(&self, other: &Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

struct LineIntersectionCalculator {
    line_segments_one: LineSegments,
    line_segments_two: LineSegments,
}

impl LineIntersectionCalculator {
    pub fn new(
        line_one: Vec<LineDirections>,
        line_two: Vec<LineDirections>,
    ) -> LineIntersectionCalculator {
        let line_segments_one = LineSegments::from_line_directions(line_one);
        let line_segments_two = LineSegments::from_line_directions(line_two);

        LineIntersectionCalculator {
            line_segments_one,
            line_segments_two,
        }
    }

    fn calculate_minimal_manhattan_distance_intersection(&self) -> u32 {
        let intersections = self.calculate_intersections();

        intersections
            .iter()
            .map(|point_with_distance| point_with_distance.point)
            .map(|point| point.manhattan_distance())
            .min()
            .unwrap_or(0)
    }

    fn calculate_minimal_steps_intersection(&self) -> u32 {
        let intersections = self.calculate_intersections();

        intersections
            .iter()
            .map(|point_with_distance| point_with_distance.distance)
            .min()
            .unwrap_or(0)
    }

    fn calculate_intersections(&self) -> Vec<PointWithDistance> {
        let mut intersections = Vec::new();
        let mut steps_one = 0;
        for line_segment_one in &self.line_segments_one {
            let mut steps_two = 0;

            for line_segment_two in &self.line_segments_two {
                let new_intersections = line_segment_one.calculate_intersections(line_segment_two);
                intersections.extend(
                    new_intersections
                        .into_iter()
                        .filter(|point| Point::new(0, 0) != *point)
                        .map(|point| {
                            let additional_steps_one =
                                point.minus(&line_segment_one.start).manhattan_distance();
                            let additional_steps_two =
                                point.minus(&line_segment_two.start).manhattan_distance();
                            PointWithDistance {
                                point,
                                distance: steps_one + steps_two + additional_steps_one + additional_steps_two,
                            }
                        }),
                );

                steps_two += line_segment_two.length();
            }

            steps_one += line_segment_one.length();
        }
        intersections
    }
}

struct PointWithDistance {
    point: Point,
    distance: u32,
}

pub fn calculate_minimal_distance_intersections(
    line_one: &str,
    line_two: &str,
) -> Result<u32, String> {
    let line_intersection_calculator = create_line_intersection_calculator_from_strings(line_one, line_two)?;

    Ok(line_intersection_calculator.calculate_minimal_manhattan_distance_intersection())
}

pub fn calculate_minimal_steps_intersections(line_one: &str, line_two: &str) -> Result<u32, String> {
    let line_inersection_calculator = create_line_intersection_calculator_from_strings(line_one, line_two)?;

    Ok(line_inersection_calculator.calculate_minimal_steps_intersection())
}

fn create_line_intersection_calculator_from_strings(line_one: &str, line_two: &str) -> Result<LineIntersectionCalculator, String> {
    let line_one_directions = parse_line_directions(line_one)?;
    let line_two_directions = parse_line_directions(line_two)?;
    let line_intersection_calculator =
        LineIntersectionCalculator::new(line_one_directions, line_two_directions);
    Ok(line_intersection_calculator)
}

fn parse_line_directions(line: &str) -> Result<Vec<LineDirections>, String> {
    line.split(",").map(|word| parse_word(word)).collect()
}

fn parse_word(word: &str) -> Result<LineDirections, String> {
    let mut chars = word.chars();

    let line_direction_type = chars.next().ok_or("Word is empty")?;

    let distance: String = chars.collect();
    let distance =
        i32::from_str(&distance).map_err(|err| format!("Could not parse distance: {}", err))?;

    match line_direction_type {
        'R' => Ok(LineDirections::Right(distance)),
        'L' => Ok(LineDirections::Left(distance)),
        'U' => Ok(LineDirections::Up(distance)),
        'D' => Ok(LineDirections::Down(distance)),
        _ => Err(format!(
            "Unknown line direction type {}.",
            line_direction_type
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LineDirections::*;

    #[test]
    fn orthogonal_line_intersection() {
        let line_one = Line::new(Point::new(4, 4), Point::new(0, 4));
        let line_two = Line::new(Point::new(2, 2), Point::new(2, 6));

        let intersections = line_one.calculate_intersections(&line_two);

        assert_eq!(intersections, vec![Point::new(2, 4)]);
    }

    #[test]
    fn parallel_line_intersection() {
        let line_one = Line::new(Point::new(3, 3), Point::new(6, 3));
        let line_two = Line::new(Point::new(4, 2), Point::new(7, 2));

        let intersections = line_one.calculate_intersections(&line_two);

        assert_eq!(intersections, vec![]);
    }

    #[test]
    fn parallel_line_intersection_with_gap() {
        let line_one = Line::new(Point::new(3, 3), Point::new(5, 3));
        let line_two = Line::new(Point::new(6, 3), Point::new(8, 3));

        let intersections = line_one.calculate_intersections(&line_two);

        assert_eq!(intersections, vec!());
    }

    #[test]
    fn overlapping_line_intersection() {
        let line_one = Line::new(Point::new(1, 4), Point::new(5, 4));
        let line_two = Line::new(Point::new(7, 4), Point::new(2, 4));

        let intersections = line_one.calculate_intersections(&line_two);

        assert_that!(
            &intersections,
            contains(vec![
                Point::new(2, 4),
                Point::new(3, 4),
                Point::new(4, 4),
                Point::new(5, 4)
            ])
        );
    }

    #[test]
    fn test_example_one() {
        let line_intersection_calculator = get_example_input_one();

        assert_eq!(
            line_intersection_calculator.calculate_minimal_manhattan_distance_intersection(),
            159
        )
    }

    #[test]
    fn test_example_two() {
        let line_intersection_calculator = get_example_input_two();

        assert_eq!(
            line_intersection_calculator.calculate_minimal_manhattan_distance_intersection(),
            135
        )
    }

    fn get_example_input_two() -> LineIntersectionCalculator {
        let line_one = vec![
            Right(98),
            Up(47),
            Right(26),
            Down(63),
            Right(33),
            Up(87),
            Left(62),
            Down(20),
            Right(33),
            Up(53),
            Right(51),
        ];
        let line_two = vec![
            Up(98),
            Right(91),
            Down(20),
            Right(16),
            Down(67),
            Right(40),
            Up(7),
            Right(15),
            Up(6),
            Right(7),
        ];
        LineIntersectionCalculator::new(line_one, line_two)
    }

    #[test]
    fn test_example_min_steps_one() {
        let line_intersection_calculator = get_example_input_one();

        assert_eq!(
            line_intersection_calculator.calculate_minimal_steps_intersection(),
            610
        )
    }

    #[test]
    fn test_example_min_steps_two() {
        let line_intersection_calculator = get_example_input_two();

        assert_eq!(
            line_intersection_calculator.calculate_minimal_steps_intersection(),
            410
        )
    }

    fn get_example_input_one() -> LineIntersectionCalculator {
        let line_one = vec![
            Right(75),
            Down(30),
            Right(83),
            Up(83),
            Left(12),
            Down(49),
            Right(71),
            Up(7),
            Left(72),
        ];
        let line_two = vec![
            Up(62),
            Right(66),
            Up(55),
            Right(34),
            Down(71),
            Right(55),
            Down(58),
            Right(83),
        ];
        LineIntersectionCalculator::new(line_one, line_two)
    }
}
