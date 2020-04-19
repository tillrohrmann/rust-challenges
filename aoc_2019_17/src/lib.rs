use crate::MapElement::{Robot, Space, Wall};
use crate::RawElement::{Newline, Other};
use aoc_common::math::Point;
use core::fmt;
use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;
use std::io;
use std::io::Error;
use std::iter::Enumerate;
use std::rc::Rc;
use std::slice::Iter;

pub struct Scaffolding {
    program: Vec<i64>,
}

impl Scaffolding {
    pub fn new(program: &Vec<i64>) -> Scaffolding {
        Scaffolding {
            program: program.clone(),
        }
    }

    pub fn extract_scaffolding(&self) -> Result<ScaffoldingMap, String> {
        let mut scaffolding_map_reader = Rc::new(RefCell::new(IntegerCollector::new()));
        let mut output_reader = aoc_2019_13::IntComputerOutputReader::new(Box::new(
            IntegerReader::new(Rc::clone(&scaffolding_map_reader)),
        ));

        let mut computer = aoc_2019_2::IntComputer::new(
            self.program.clone(),
            io::BufReader::new(io::stdin()),
            &mut output_reader,
        );

        computer.compute();

        let scaffolding = scaffolding_map_reader.borrow();
        let raw_map_input = scaffolding.get_output();

        raw_map_input.try_into()
    }
}

pub struct ScaffoldingMap {
    map: Vec<MapElement>,
    width: usize,
    offset: Point,
}

impl ScaffoldingMap {
    pub fn new(elements: Vec<MapElement>, width: usize) -> ScaffoldingMap {
        ScaffoldingMap {
            map: elements,
            width,
            offset: Point(0, 0),
        }
    }

    pub fn at(&self, point: Point) -> Option<MapElement> {
        let Point(x, y) = point - self.offset;

        if x < 0 || y < 0 {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<MapElement> {
        self.map.get(x + y * self.width).cloned()
    }

    pub fn neighbours(&self, point: Point) -> Vec<MapElementItem> {
        let east = point + Point(1, 0);
        let west = point + Point(-1, 0);
        let south = point + Point(0, 1);
        let north = point + Point(0, -1);
        vec![east, west, south, north]
            .into_iter()
            .map(|point| self.at(point).map(|v| MapElementItem::new(point, v)))
            .flatten()
            .collect()
    }

    pub fn elements(&self) -> ElementIterator {
        ElementIterator::new(self)
    }
}

pub struct ElementIterator<'a> {
    element_iterator: Enumerate<Iter<'a, MapElement>>,
    width: usize,
    offset: Point,
}

impl<'a> ElementIterator<'a> {
    pub fn new(scaffolding_map: &'a ScaffoldingMap) -> ElementIterator {
        let element_iterator = scaffolding_map.map.iter().enumerate();

        ElementIterator {
            element_iterator,
            width: scaffolding_map.width,
            offset: scaffolding_map.offset,
        }
    }
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = MapElementItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.element_iterator.next().map(|(idx, &element)| {
            let x = idx % self.width;
            let y = idx / self.width;
            MapElementItem::new(Point(x as isize, y as isize) + self.offset, element)
        })
    }
}

#[derive(Debug)]
pub struct MapElementItem {
    point: Point,
    map_element: MapElement,
}

impl MapElementItem {
    pub fn new(point: Point, map_element: MapElement) -> MapElementItem {
        MapElementItem { point, map_element }
    }
}

impl TryFrom<&Vec<i64>> for ScaffoldingMap {
    type Error = String;

    fn try_from(value: &Vec<i64>) -> Result<Self, Self::Error> {
        let raw_elements: Vec<RawElement> = value
            .into_iter()
            .map(|&x| x.try_into())
            .collect::<Result<Vec<RawElement>, String>>()?;

        let first_newline = raw_elements.iter().position(|r| r.eq(&Newline));

        let width = first_newline.unwrap_or(raw_elements.len());
        let elements: Vec<MapElement> = raw_elements
            .into_iter()
            .filter_map(|element| match element {
                Newline => None,
                Other(map_element) => Some(map_element),
            })
            .collect();

        Ok(ScaffoldingMap::new(elements, width))
    }
}

impl fmt::Display for ScaffoldingMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.map
            .chunks(self.width)
            .map(|line| {
                let str_line: String = line.iter().map(|e| e.to_string()).collect();
                writeln!(f, "{}", str_line)
            })
            .collect()
    }
}

#[derive(PartialEq)]
enum RawElement {
    Other(MapElement),
    Newline,
}

impl TryFrom<i64> for RawElement {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(RawElement::Newline),
            x => {
                let map_element: MapElement = x.try_into()?;
                Ok(Other(map_element))
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MapElement {
    Wall,
    Space,
    Robot,
    Intersection,
}

impl ToString for MapElement {
    fn to_string(&self) -> String {
        match self {
            Wall => "#",
            Space => ".",
            Robot => "R",
            Intersection => "O",
        }
        .into()
    }
}

impl TryFrom<i64> for MapElement {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            35 => Ok(Wall),
            46 => Ok(Space),
            94 => Ok(Robot),
            x => Err(format!("Cannot parse MapElement from {}.", x)),
        }
    }
}

pub struct IntegerCollector {
    values: Vec<i64>,
}

impl IntegerCollector {
    pub fn new() -> IntegerCollector {
        IntegerCollector { values: vec![] }
    }

    pub fn get_output(&self) -> &Vec<i64> {
        &self.values
    }

    fn push(&mut self, value: i64) {
        self.values.push(value);
    }
}

pub struct IntegerReader {
    collector: Rc<RefCell<IntegerCollector>>,
}

impl IntegerReader {
    pub fn new(collector: Rc<RefCell<IntegerCollector>>) -> IntegerReader {
        IntegerReader { collector }
    }
}

impl aoc_2019_13::OutputReader for IntegerReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        let value = output_value
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        self.collector.borrow_mut().push(value);

        Ok(())
    }

    fn finalize_input_sequence(&self) {}
}

pub fn find_intersections(input_map: &ScaffoldingMap) -> Vec<Point> {
    let intersections: Vec<Point> = input_map
        .elements()
        .into_iter()
        .filter_map(|item| {
            if is_accessible(item.map_element) {
                Some(item.point)
            } else {
                None
            }
        })
        .filter(|&p| {
            input_map
                .neighbours(p)
                .iter()
                .filter(|&i| is_accessible(i.map_element))
                .count()
                > 2
        })
        .collect();

    intersections
}

fn is_accessible(map_element: MapElement) -> bool {
    map_element != MapElement::Space
}

#[cfg(test)]
mod tests {
    use super::*;
}
