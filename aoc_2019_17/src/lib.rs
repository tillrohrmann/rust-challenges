use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::Error;
use std::rc::Rc;
use crate::MapElement::{Wall, Space, Robot};
use crate::RawElement::{Other, Newline};
use core::fmt;
use std::fmt::Formatter;

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
}

impl ScaffoldingMap {
    pub fn new(elements: Vec<MapElement>, width: usize) -> ScaffoldingMap {
        ScaffoldingMap {
            map: elements,
            width,
        }
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
        let elements: Vec<MapElement> = raw_elements.into_iter().filter_map(|element| {
            match element {
                Newline => None,
                Other(map_element) => Some(map_element),
            }
        }).collect();

        Ok(ScaffoldingMap::new(elements, width))
    }
}

impl fmt::Display for ScaffoldingMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.map.chunks(self.width).map(|line| {
            let str_line: String = line.iter().map(|e| e.to_string()).collect();
            writeln!(f, "{}", str_line)
        }).collect()
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

#[derive(PartialEq)]
pub enum MapElement {
    Wall,
    Space,
    Robot,
}

impl ToString for MapElement {
    fn to_string(&self) -> String {
        match self {
            Wall => "#",
            Space => ".",
            Robot => "R",
        }.into()
    }
}

impl TryFrom<i64> for MapElement {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            35 => Ok(Wall),
            46 => Ok(Space),
            94 => Ok(Robot),
            x => Err(format!("Cannot parse MapElement from {}.", x))
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

#[cfg(test)]
mod tests {

    use super::*;
}
