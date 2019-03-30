use core::fmt;
use core::fmt::Write;
use std::error::Error;
use std::fmt::{Display, Formatter};

use aoc_common::{GenericResult, read_raw_file_content, GenericError};
use aoc_common::math::Point;
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashMap, HashSet};

type Direction = Point;

#[derive(PartialEq, Eq, Clone)]
pub struct Car {
    position: Point,
    direction: Direction,
    crossings_counter: usize,
}

impl Car {
    pub fn new(position: Point, direction: Point, crossings_counter: usize) -> Car {
        Car {
            position,
            direction,
            crossings_counter,
        }
    }

    fn advance(&self) -> Car {
        Car::new(self.position + self.direction, self.direction, self.crossings_counter)
    }

    fn turn_left(&self) -> Car {
        let Point(v_x, v_y) = self.direction;
        let new_direction = Point(-1 * v_y, -1 * v_x);
        Car::new(self.position + new_direction, new_direction, self.crossings_counter)
    }

    fn turn_right(&self) -> Car {
        let Point(v_x, v_y) = self.direction;
        let new_direction = Point(v_y, v_x);
        Car::new(self.position + new_direction, new_direction, self.crossings_counter)
    }

    fn crossing(&self) -> Car {
        let Point(v_x, v_y) = self.direction;
        let new_direction = match self.crossings_counter {
            0 => Point(v_y, -1 * v_x),
            1 => Point(v_x, v_y),
            2 => Point(-1 * v_y, v_x),
            _ => panic!("Encountered invalid crossings counter {}", self.crossings_counter)
        };

        Car::new(self.position + new_direction, new_direction, (self.crossings_counter + 1) % 3)
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.direction {
            Point(1, 0) => f.write_char('>'),
            Point(-1, 0) => f.write_char('<'),
            Point(0, 1) => f.write_char('v'),
            Point(0, -1) => f.write_char('^'),
            _ => Err(fmt::Error)
        }

    }
}

enum StreetElement {
    HorizontalRoad,
    VerticalRoad,
    Crossing,
    LeftTurn,
    RightTurn,
    Empty,
}

impl StreetElement {
    fn advance_car(&self, car: &Car) -> Car {
        match self {
            StreetElement::HorizontalRoad | StreetElement::VerticalRoad => car.advance(),
            StreetElement::LeftTurn => car.turn_left(),
            StreetElement::RightTurn => car.turn_right(),
            StreetElement::Crossing => car.crossing(),
            StreetElement::Empty => panic!("Car cannot be on empty ground."),
        }
    }

    fn parse_street_element(element: char) -> GenericResult<StreetElement> {
        match element {
            ' ' => Ok(StreetElement::Empty),
            '+' => Ok(StreetElement::Crossing),
            '-' => Ok(StreetElement::HorizontalRoad),
            '|' => Ok(StreetElement::VerticalRoad),
            '/' => Ok(StreetElement::LeftTurn),
            '\\' => Ok(StreetElement::RightTurn),
            _ => Err(GenericError::new(&format!("Unknown street element {}", element)).into())
        }
    }
}

impl Display for StreetElement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            StreetElement::HorizontalRoad => f.write_char('-'),
            StreetElement::VerticalRoad => f.write_char('|'),
            StreetElement::Crossing => f.write_char('+'),
            StreetElement::LeftTurn => f.write_char('/'),
            StreetElement::RightTurn => f.write_char('\\'),
            StreetElement::Empty => f.write_char(' '),
        }
    }
}

pub struct StreetMap {
    map: Vec<Vec<StreetElement>>,
    cars: Vec<Car>,
}

enum CarLookup<'a> {
    Collision,
    Car(&'a Car),
    Empty,
}

#[derive(PartialEq, Eq)]
struct MinCar<'a>(&'a Car);

impl<'a> PartialOrd for MinCar<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for MinCar<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let &MinCar(other_car) = other;
        let &MinCar(car) = self;

        let Point(other_x, other_y) = other_car.position;
        let Point(x, y) = car.position;

        match other_y.cmp(&y) {
            std::cmp::Ordering::Equal => other_x.cmp(&x),
            x => x,
        }
    }
}

impl StreetMap {
    fn new(map: Vec<Vec<StreetElement>>, cars: Vec<Car>) -> StreetMap {
        StreetMap {
            map,
            cars,
        }
    }

    fn get_car(&self, point: Point) -> CarLookup {
        let result = self.cars.iter().filter(|&car| car.position == point).collect::<Vec<&Car>>();

        if result.is_empty() {
            CarLookup::Empty
        } else if result.len() == 1 {
            CarLookup::Car(result[0])
        } else {
            CarLookup::Collision
        }
    }

    pub fn advance(&mut self) {
        let mut cars_in_order = BinaryHeap::with_capacity(self.cars.len());
        let mut new_positions = HashSet::with_capacity(self.cars.len());
        let collisions = self.check_collision();
        let mut new_cars: Vec<Car> = Vec::with_capacity(self.cars.len());

        for car in self.cars.iter() {
            cars_in_order.push(MinCar(car));
        }

        while let Some(MinCar(car)) = cars_in_order.pop() {
            let new_car = if new_positions.contains(&car.position) || collisions.contains(&car.position) {
                car.clone()
            } else {
                let Point(x, y) = car.position;
                self.map[y as usize][x as usize].advance_car(car)
            };

            new_positions.insert(new_car.position);
            new_cars.push(new_car);
        }

        self.cars = new_cars;
    }

    pub fn check_collision(&self) -> Vec<Point> {
        let mut counter = HashMap::with_capacity(self.cars.len());
        for car in self.cars.iter() {
            *counter.entry(&car.position).or_insert(0) += 1;
        }

        counter.iter().filter(|&(&point, &count)| count > 1).map(|(&&p, count)| p).collect()
    }
}

impl Display for StreetMap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (row_index, row) in self.map.iter().enumerate() {
            for (column_index, street_element) in row.iter().enumerate() {
                match self.get_car(Point(column_index as isize, row_index as isize)) {
                    CarLookup::Car(car) => car.fmt(f)?,
                    CarLookup::Collision => f.write_char('X')?,
                    CarLookup::Empty => street_element.fmt(f)?,
                };
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

pub fn read_street_map(path: &str) -> GenericResult<StreetMap> {
    let content = read_raw_file_content(path)?;
    let mut map = Vec::with_capacity(content.len());
    let mut cars = Vec::with_capacity(10);

    for (row_index, row) in content.iter().enumerate() {
        let (street_elements, partial_cars) = parse_map_row(row)?;
        map.push(street_elements);

        for PartialCar(x, direction) in partial_cars {
            cars.push(Car::new(Point(x, row_index as isize), direction, 0))
        }
    }
    Ok(StreetMap::new(map, cars))
}

struct PartialCar(isize, Direction);

fn parse_map_row(row: &str) -> GenericResult<(Vec<StreetElement>, Vec<PartialCar>)> {
    let mut street_elements = Vec::with_capacity(row.len());
    let mut partial_cars = Vec::with_capacity(1);

    for (index, element) in row.chars().enumerate() {
        let (street_element, optional_direction) = parse_street_element(element)?;
        street_elements.push(street_element);
        optional_direction.map(|direction| partial_cars.push(PartialCar(index as isize, direction)));
    }

    Ok((street_elements, partial_cars))
}

fn parse_street_element(element: char) -> GenericResult<(StreetElement, Option<Direction>)> {
    let (element, optional_direction) = match element {
        '>' => ('-', Some(Point(1, 0))),
        '<' => ('-', Some(Point(-1, 0))),
        '^' => ('|', Some(Point(0, -1))),
        'v' => ('|', Some(Point(0, 1))),
        _ => (element, None),
    };

    StreetElement::parse_street_element(element).map(|street_element| (street_element, optional_direction))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
