#[macro_use] extern crate lazy_static;
use aoc_common::{GenericResult, GenericError};
use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::collections::vec_deque::VecDeque;
use std::cmp::Ordering;

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

#[derive(Debug, PartialOrd, PartialEq, Clone)]
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
    const EMPTY_CHAR: char = '.';
    const POINT_CHAR: char = '#';
    const ORIGIN: Vector = Vector{ x: 0, y: 0};

    pub fn new(points_with_velocity: Vec<PointWithVelocity>) -> PointMap {
        let points = points_with_velocity.iter().map(|p| p.position).collect();
        let (min_x, max_x) = PointMap::find_min_max(&points, |p| p.x);
        let (min_y, max_y) = PointMap::find_min_max(&points, |p| p.y);
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

    fn find_min_max(points_with_velocity: &Vec<Vector>, field_selector: fn(&Vector) -> isize) -> (isize, isize) {
        let min = points_with_velocity.iter().map(field_selector).min().unwrap_or(0);
        let max = points_with_velocity.iter().map(field_selector).max().unwrap_or(0);

        (min, max)
    }

    pub fn display(&self) {
        let mut points: Vec<Vector> = self.points_iter().collect::<HashSet<Vector>>().into_iter().collect();

        points.sort_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        dbg!(&points);

        let (min_x, max_x) = PointMap::find_min_max(&points, |p| p.x);
        let (min_y, max_y) = PointMap::find_min_max(&points, |p| p.y);

        let size = dbg!(Vector::new(max_x - min_x + 1, max_y - min_y + 1));
        let origin = dbg!(Vector::new(min_x, min_y));

        let mut current_position = Vector::new(0, 0);

        for point in points.iter() {
            let normalized_point = *point - origin;
            while current_position.y < normalized_point.y {
                for _ in current_position.x..size.x {
                    print!("{}", PointMap::EMPTY_CHAR);
                }
                println!();
                current_position.y += 1;
                current_position.x = 0;
                current_position;
            }

            while current_position.x < normalized_point.x {
                print!("{}", PointMap::EMPTY_CHAR);
                current_position.x += 1;
            }

            print!("{}", PointMap::POINT_CHAR);
            current_position.x += 1;
        }
    }

    pub fn advance(&mut self) {
        for point in self.points_with_velocity.iter_mut() {
            point.position += point.velocity;
        }

        // filter out points which have left the valid map
        self.points_with_velocity = self.points_with_velocity.iter()
            .filter(|point_with_velocity| {
                let position = point_with_velocity.position;
                PointMap::ORIGIN <= position && position < self.size
            })
            .map(|p| p.clone())
            .collect();
    }

    pub fn points_iter<'a>(&'a self) -> impl Iterator<Item = Vector> + 'a {
        self.points_with_velocity.iter().map(|point_with_velocity| point_with_velocity.position)
    }

    pub fn number_points(&self) -> usize {
        self.points_with_velocity.len()
    }

    pub fn size(&self) -> Vector {
        self.size
    }
}

pub struct InitialComponents {
    components: Vec<Vector>,
    edges: HashMap<Vector, Vec<Vector>>,
    size: Vector,
}

pub struct ConnectedComponents {
    connected_components: HashMap<usize, usize>,
}

impl InitialComponents {
    pub fn new(components: Vec<Vector>, size: Vector) -> InitialComponents {
        let mut edges : HashMap<Vector, Vec<Vector>>= components.iter().cloned().map(|vector| (vector, vec![])).collect();

        for point in components.iter() {
            InitialComponents::check_edge_exists(&mut edges, point, Vector::new(point.x + 1, point.y));
            InitialComponents::check_edge_exists(&mut edges, point, Vector::new(point.x - 1, point.y));
            InitialComponents::check_edge_exists(&mut edges, point, Vector::new(point.x, point.y + 1));
            InitialComponents::check_edge_exists(&mut edges, point, Vector::new(point.x, point.y - 1));
        }

        InitialComponents {
            components,
            edges,
            size,
        }
    }

    fn check_edge_exists(edges: &mut HashMap<Vector, Vec<Vector>>, point: &Vector, xp: Vector) -> () {
        if edges.contains_key(&xp) {
            edges.entry(*point).and_modify(|edges| edges.push(xp));
        }
    }

    pub fn calculate_connected_components(mut self) -> ConnectedComponents {
        let mut queue = self.components.iter().cloned().collect::<VecDeque<Vector>>();
        let mut map: HashMap<Vector, usize> = self.components.iter().enumerate().map(|(index, point)| (point.clone(), index)).collect();

        while let Some(point) = queue.pop_front() {
            let neighbours = self.edges.get(&point).unwrap();

            let min_neighbour = InitialComponents::find_min_neighbour(&map, neighbours);

            if let Some(neighbour) = min_neighbour {
                map.entry(point).and_modify(|value| {
                    if neighbour < *value {
                        *value = neighbour;

                        for neighbour in neighbours.iter() {
                            queue.push_back(neighbour.clone());
                        }
                    }
                });
            }
        }

        let mut connected_components : HashMap<usize, usize>= HashMap::new();

        for &connected_component in map.values() {
            *connected_components.entry(connected_component).or_insert(0) += 1;
        }

        ConnectedComponents::new(connected_components)
    }

    fn find_min_neighbour(map: &HashMap<Vector, usize>, neighbours: &Vec<Vector>) -> Option<usize> {
        neighbours
            .iter()
            .map(|neighbour| map.get(neighbour).cloned())
            .flatten()
            .min()
    }
}

impl ConnectedComponents {
    pub fn new(connected_components: HashMap<usize, usize>) -> ConnectedComponents {
        ConnectedComponents {
            connected_components,
        }
    }

    pub fn number_connected_components(&self) -> usize {
        self.connected_components.len()
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
