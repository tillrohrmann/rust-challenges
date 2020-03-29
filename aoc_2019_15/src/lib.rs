use aoc_common::math::Point;
use rand::Rng;
use std::cell::RefCell;
use std::fmt::{Formatter, Write};
use std::io::{Error, ErrorKind};
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;
use std::{error, fmt, io, num};
use std::time::Duration;

pub struct DroidProgram {
    program: Vec<i64>,
}

impl DroidProgram {
    pub fn new(program: Vec<i64>) -> DroidProgram {
        DroidProgram { program }
    }

    pub fn run(&mut self) {
        let controller = Rc::new(RefCell::new(DroidController::new()));
        let mut output = aoc_2019_13::IntComputerOutputReader::new(Box::new(
            DroidOutputReader::new(Rc::clone(&controller)),
        ));
        let input = io::BufReader::new(DroidDirectionController::new(Rc::clone(&controller)));
        let mut computer = aoc_2019_2::IntComputer::new(self.program.clone(), input, &mut output);

        computer.compute();
    }
}

#[derive(Copy, Clone)]
enum DroidMapElement {
    Wall,
    Oxygen,
    Floor,
    Unknown,
}

impl fmt::Display for DroidMapElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chr = match self {
            DroidMapElement::Wall => '#',
            DroidMapElement::Oxygen => 'O',
            DroidMapElement::Floor => '.',
            DroidMapElement::Unknown => ' ',
        };

        f.write_char(chr)
    }
}

struct MapSize {
    width: usize,
    height: usize,
}

impl MapSize {
    fn new(width: usize, height: usize) -> MapSize {
        MapSize { width, height }
    }
}

struct DroidMap {
    offset: Point,
    size: MapSize,
    map: Vec<DroidMapElement>,
}

impl DroidMap {
    fn new() -> DroidMap {
        let offset = Point(0, 0);
        let size = MapSize::new(16, 16);
        let map = vec![DroidMapElement::Unknown; size.width * size.height];

        DroidMap { offset, size, map }
    }

    fn draw(&self, droid_position: Point) {
        let droid_position = droid_position - self.offset;
        self.map
            .chunks(self.size.width)
            .enumerate()
            .for_each(|(row_idx, chunk)| self.draw_line(row_idx, chunk, droid_position));
    }

    fn draw_line(&self, y: usize, line: &[DroidMapElement], droid_position: Point) {
        for (x, map_element) in line.iter().enumerate() {
            if droid_position == Point(x as isize, y as isize) {
                print!("D");
            } else {
                print!("{}", map_element);
            }
        }

        println!();
    }

    fn update(&mut self, position: Point, element: DroidMapElement) {
        self.resize_map_if_required(position);
        let Point(x, y) = position - self.offset;

        self.map[x as usize + y as usize * self.size.width] = element;
    }

    fn resize_map_if_required(&mut self, position: Point) {
        let Point(x, y) = position;
        let Point(x_offset, y_offset) = self.offset;
        let Point(x_max, y_max) =
            self.offset + Point(self.size.width as isize, self.size.height as isize);

        let Point(new_x_offset, new_y_offset) = Point(x_offset.min(x), y_offset.min(y));
        let Point(new_x_max, new_y_max) = Point(x_max.max(x), y_max.max(y));
        let new_map_size = MapSize::new(
            (new_x_max - new_x_offset) as usize,
            (new_y_max - new_y_offset) as usize,
        );

        let mut new_map = vec![DroidMapElement::Unknown; new_map_size.width * new_map_size.height];

        let lines_to_skip: usize = (y_offset - new_y_offset) as usize;
        let cols_to_skip: usize = (x_offset - new_x_offset) as usize;
        let end_column = self.size.width + cols_to_skip;

        new_map
            .chunks_mut(new_map_size.width)
            .skip(lines_to_skip)
            .zip(self.map.chunks(self.size.width))
            .for_each(|(new, old)| {
                new[cols_to_skip..(end_column)]
                    .copy_from_slice(old)
            });

        self.offset = Point(new_x_offset, new_y_offset);
        self.size = new_map_size;
        self.map = new_map;
    }
}

struct DroidController {
    droid_map: DroidMap,
    droid_position: Point,
    current_direction: DroidDirection,
}

impl DroidController {
    fn new() -> DroidController {
        DroidController {
            droid_map: DroidMap::new(),
            droid_position: Point(0, 0),
            current_direction: DroidDirection::North,
        }
    }

    fn next_droid_direction(&mut self) -> DroidDirection {
        // let mut input = String::new();
        // io::stdin().read_line(&mut input);
        //
        // let next_direction = input.trim().parse::<usize>().unwrap();
        let next_direction = rand::thread_rng().gen_range(1, 5);

        self.current_direction = match next_direction {
            1 => DroidDirection::North,
            2 => DroidDirection::South,
            3 => DroidDirection::West,
            4 => DroidDirection::East,
            _ => panic!("Invalid random number"),
        };

        self.current_direction
    }

    fn update_droid_status(&mut self, droid_status: DroidStatus) {
        match droid_status {
            DroidStatus::Wall => self.hit_wall(),
            DroidStatus::Moved => self.moved(),
            DroidStatus::Oxygen => self.found_oxygen(),
        }
    }

    fn hit_wall(&mut self) {
        let position = self.current_direction.translate(self.droid_position);
        self.droid_map.update(position, DroidMapElement::Wall);
    }

    fn moved(&mut self) {
        let position = self.current_direction.translate(self.droid_position);
        self.droid_map.update(position, DroidMapElement::Floor);
        self.droid_position = position;
    }

    fn found_oxygen(&mut self) {
        let position = self.current_direction.translate(self.droid_position);
        self.droid_map.update(position, DroidMapElement::Oxygen);
        self.droid_position = position;
    }

    fn draw(&self) {
        self.droid_map.draw(self.droid_position);
    }
}

struct DroidOutputReader {
    droid_controller: Rc<RefCell<DroidController>>,
}

#[derive(Debug)]
struct DroidStatusParseError {
    inner: Box<dyn error::Error + Send + Sync>,
}

impl DroidStatusParseError {
    fn new<E>(inner: E) -> DroidStatusParseError
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        DroidStatusParseError {
            inner: inner.into(),
        }
    }
}

impl fmt::Display for DroidStatusParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse the droid status because {}", self.inner)
    }
}

impl error::Error for DroidStatusParseError {}

impl From<num::ParseIntError> for DroidStatusParseError {
    fn from(error: ParseIntError) -> Self {
        DroidStatusParseError::new(Box::new(error))
    }
}

impl From<String> for DroidStatusParseError {
    fn from(inner: String) -> Self {
        DroidStatusParseError::new(inner)
    }
}

impl From<DroidStatusParseError> for io::Error {
    fn from(inner: DroidStatusParseError) -> Self {
        io::Error::new(ErrorKind::InvalidInput, inner)
    }
}

enum DroidStatus {
    Wall,
    Moved,
    Oxygen,
}

impl FromStr for DroidStatus {
    type Err = DroidStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<usize>()?;

        match value {
            0 => Ok(DroidStatus::Wall),
            1 => Ok(DroidStatus::Moved),
            2 => Ok(DroidStatus::Oxygen),
            x => Err(DroidStatusParseError::new(format!(
                "Could not parse droid status with code {}.",
                x
            ))),
        }
    }
}

impl DroidOutputReader {
    fn new(droid_controller: Rc<RefCell<DroidController>>) -> DroidOutputReader {
        DroidOutputReader { droid_controller }
    }
}

impl aoc_2019_13::OutputReader for DroidOutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        let droid_status = output_value.parse::<DroidStatus>()?;
        self.droid_controller
            .borrow_mut()
            .update_droid_status(droid_status);
        Ok(())
    }

    fn finalize_input_sequence(&self) {
        self.droid_controller.borrow().draw();
    }
}

#[derive(Copy, Clone)]
enum DroidDirection {
    North,
    West,
    South,
    East,
}

impl DroidDirection {
    fn translate(self, position: Point) -> Point {
        match self {
            DroidDirection::North => position + Point(0, -1),
            DroidDirection::South => position + Point(0, 1),
            DroidDirection::West => position + Point(-1, 0),
            DroidDirection::East => position + Point(1, 0),
        }
    }
}

struct DroidDirectionController {
    controller: Rc<RefCell<DroidController>>,
}

impl DroidDirectionController {
    fn new(controller: Rc<RefCell<DroidController>>) -> DroidDirectionController {
        DroidDirectionController { controller }
    }

    fn translate_into_command(direction: DroidDirection) -> Vec<u8> {
        let command = match direction {
            DroidDirection::North => "1\n",
            DroidDirection::South => "2\n",
            DroidDirection::West => "3\n",
            DroidDirection::East => "4\n",
        };

        command.into()
    }
}

impl io::Read for DroidDirectionController {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let direction = self.controller.borrow_mut().next_droid_direction();

        let command = DroidDirectionController::translate_into_command(direction);
        (&command[..]).read(buf)
    }
}

pub fn create_droid_program_from_input(path: &str) -> DroidProgram {
    let program = aoc_2019_2::read_memory_from_file(path);

    DroidProgram::new(program)
}

#[cfg(test)]
mod tests {
    use super::*;
}
