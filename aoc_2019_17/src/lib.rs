use crate::CommandSequence::{Comma, Command};
use crate::Direction::{East, North, South, West};
use crate::MapElement::{Char, Robot, Space, Wall};
use crate::RawElement::{Newline, Other};
use aoc_common::math::Point;
use core::fmt;
use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Formatter, Debug};
use std::io;
use std::io::{Error, ErrorKind};
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
    Robot(Option<Direction>),
    Char(char),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            North => "^",
            East => ">",
            South => "v",
            West => "<",
        }
        .into()
    }
}

impl ToString for MapElement {
    fn to_string(&self) -> String {
        match self {
            Wall => "#".into(),
            Space => ".".into(),
            Robot(direction) => direction.map(|d| d.to_string()).unwrap_or("X".into()),
            Char(chr) => chr.to_string(),
        }
    }
}

impl TryFrom<i64> for MapElement {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match char::from(value as u8) {
            '#' => Ok(Wall),
            '.' => Ok(Space),
            '^' => Ok(Robot(Some(North))),
            '>' => Ok(Robot(Some(East))),
            '<' => Ok(Robot(Some(West))),
            'v' => Ok(Robot(Some(South))),
            'X' => Ok(Robot(None)),
            x => Ok(Char(x)),
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

    fn clear(&mut self) {
        self.values.clear();
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

    fn finalize_input_sequence(&mut self) {}
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

pub struct VacuumCleaner {
    program: Vec<i64>,
}

impl VacuumCleaner {
    pub fn new(program: &Vec<i64>) -> Self {
        Self {
            program: program.clone(),
        }
    }

    pub fn execute(&mut self) {
        let vacuum_cleaner_controller = Rc::new(RefCell::new(VacuumCleanerController::new()));
        let mut output = aoc_2019_13::IntComputerOutputReader::new(Box::new(
            VacuumCleanerDisplay::new(Rc::clone(&vacuum_cleaner_controller)),
        ));
        let input = VacuumController::new(Rc::clone(&vacuum_cleaner_controller));

        let mut computer = aoc_2019_2::IntComputer::new(
            self.program.clone(),
            io::BufReader::new(input),
            &mut output,
        );

        computer.compute();
    }
}

#[derive(Debug, Copy, Clone)]
enum MainFunction {
    A,
    B,
    C,
}

impl ToString for MainFunction {
    fn to_string(&self) -> String {
        match self {
            MainFunction::A => 'A'.to_string(),
            MainFunction::B => 'B'.to_string(),
            MainFunction::C => 'C'.to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Function {
    Left,
    Right,
    Move(u32),
}

impl ToString for Function {
    fn to_string(&self) -> String {
        match self {
            Function::Left => 'L'.to_string(),
            Function::Right => 'R'.to_string(),
            Function::Move(length) => format!("{}", length),
        }
    }
}

struct VacuumCleanerController {}

impl VacuumCleanerController {
    fn new() -> Self {
        Self {}
    }

    fn get_main_command(&self) -> Vec<MainFunction> {
        vec![MainFunction::A, MainFunction::B, MainFunction::A, MainFunction::C, MainFunction::A, MainFunction::B, MainFunction::C, MainFunction::B, MainFunction::C, MainFunction::A]
    }

    fn get_a_function(&self) -> Vec<Function> {
        vec![Function::Left, Function::Move(12), Function::Right, Function::Move(4), Function::Right, Function::Move(4), Function::Left, Function::Move(6)]
    }

    fn get_b_function(&self) -> Vec<Function> {
        vec![Function::Left, Function::Move(12), Function::Right, Function::Move(4), Function::Right, Function::Move(4), Function::Right, Function::Move(12)]
    }

    fn get_c_function(&self) -> Vec<Function> {
        vec![Function::Left, Function::Move(10), Function::Left, Function::Move(6), Function::Right, Function::Move(4)]
    }
}

struct VacuumCleanerDisplay {
    collector: Option<Vec<i64>>,
    cleaner: Rc<RefCell<VacuumCleanerController>>,
}

impl VacuumCleanerDisplay {
    fn new(cleaner: Rc<RefCell<VacuumCleanerController>>) -> Self {
        Self {
            collector: Some(vec![]),
            cleaner,
        }
    }
}

impl aoc_2019_13::OutputReader for VacuumCleanerDisplay {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        let value: i64 = output_value
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        self.collector.as_mut().map(|v| v.push(value));

        VacuumCleanerDisplay::display_value(value);

        Ok(())
    }

    fn finalize_input_sequence(&mut self) {
        let result: Option<Result<ScaffoldingMap, String>> =
            self.collector.take().map(|v| (&v).try_into());
    }
}

#[derive(Debug)]
enum CommandSequence<T> {
    Command(T),
    Comma,
    Newline,
}

impl<T> ToString for &CommandSequence<T> where T: ToString + Copy {
    fn to_string(&self) -> String {
        match self {
            CommandSequence::Comma => ','.to_string(),
            CommandSequence::Newline => '\n'.to_string(),
            CommandSequence::Command(c) => (*c).to_string(),
        }
    }
}

struct VacuumController {
    cleaner: Rc<RefCell<VacuumCleanerController>>,
    command: Option<(Vec<u8>, usize)>,
    video_feed: bool,
}

impl VacuumController {
    fn new(cleaner: Rc<RefCell<VacuumCleanerController>>) -> Self {
        Self {
            cleaner,
            command: None,
            video_feed: false,
        }
    }

    fn get_vacuum_controller_command(&self) -> (Vec<u8>, usize) {
        let main_command = self.cleaner.borrow().get_main_command();
        let a_function = self.cleaner.borrow().get_a_function();
        let b_function = self.cleaner.borrow().get_b_function();
        let c_function = self.cleaner.borrow().get_c_function();
        let video_feed = self.get_video_feed();

        let main_command = VacuumController::translate_main_function(&main_command);
        let a_function = VacuumController::translate_function(&a_function);
        let b_function = VacuumController::translate_function(&b_function);
        let c_function = VacuumController::translate_function(&c_function);

        let continuous_video_feed = VacuumController::translate_video_feed(video_feed);
        let mut result: Vec<u8> = Vec::new();
        result.extend(main_command);
        result.extend(a_function);
        result.extend(b_function);
        result.extend(c_function);
        result.extend(continuous_video_feed);

        (result, 0)
    }

    fn get_video_feed(&self) -> bool {
        self.video_feed
    }

    fn translate_video_feed(video_feed: bool) -> Vec<u8> {
        let video_command = if video_feed {
            vec!["y"]
        } else {
            vec!["n"]
        };
        let command_sequence = VacuumController::translate_into_command_sequence(&video_command);
        VacuumController::translate_into_raw_sequence(&command_sequence)
    }

    fn translate_main_function(main_command: &Vec<MainFunction>) -> Vec<u8> {
        let command_sequence: Vec<CommandSequence<MainFunction>> =
            VacuumController::translate_into_command_sequence(main_command);
        VacuumController::translate_into_raw_sequence(&command_sequence)
    }

    fn translate_into_command_sequence<T: Debug + Copy>(main_command: &Vec<T>) -> Vec<CommandSequence<T>> {
        let mut result: Vec<CommandSequence<T>> = main_command
            .into_iter()
            .flat_map(|&command| vec![Command(command), Comma])
            .collect();
        *result.last_mut().unwrap() = CommandSequence::Newline;

        result
    }

    fn translate_into_raw_sequence<T>(command_sequence: &Vec<CommandSequence<T>>) -> Vec<u8> where T: Copy + ToString {
        let result: Vec<u8> = command_sequence
            .into_iter()
            .flat_map(|command| {
                let str: String = command.to_string();
                let result: Vec<u8> = str.chars().flat_map(|chr| {
                    let numeric_value = chr as u8;
                    let digits: Vec<u8> = VacuumController::split_into_digits(numeric_value);

                    let mut foobar: Vec<char> = digits
                        .into_iter()
                        .map(|digit| char::from('0' as u8 + digit))
                        .collect();
                    foobar.push('\n');

                    foobar.into_iter().map(|c| c as u8).collect::<Vec<u8>>()
                }).collect();

                result
            })
            .collect();

        result
    }

    fn translate_function(function: &Vec<Function>) -> Vec<u8> {
        let command_sequence = VacuumController::translate_into_command_sequence(function);
        VacuumController::translate_into_raw_sequence(&command_sequence)
    }

    fn split_into_digits(mut value: u8) -> Vec<u8> {
        let mut result = Vec::with_capacity(3);

        while value > 0 {
            result.push(value % 10);
            value /= 10;
        }

        result.reverse();

        result
    }
}

impl io::Read for VacuumController {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.command.is_none() {
            let (command, offset) = self.get_vacuum_controller_command();
            self.command = Some((command, offset));
        };

        let (command, offset) = self.command.as_mut().unwrap();

        let result = (&command[*offset..]).read(buf);

        if let Some(&bytes_read) = result.as_ref().ok() {
            *offset += bytes_read;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

impl VacuumCleanerDisplay {
    fn display_value(value: i64) {
        if value >= 0 && value < 256 {
            print!("{}", char::from(value as u8))
        } else {
            println!("{}", value);
        }
    }
}
