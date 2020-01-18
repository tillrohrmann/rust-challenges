use aoc_2019_7::{AsyncIntComputer, ChannelReader, ChannelWriter};
use aoc_common::math::Point;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::SendError;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn from_int(value: i64) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Cannot create Color from {}.", value),
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let current_idx = self.to_int();
        let num_directions = directions.len() as i64;
        let new_idx = (current_idx + Direction::to_offset(turn) + num_directions) % num_directions;

        directions[new_idx as usize]
    }

    fn to_int(&self) -> i64 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    fn to_offset(turn: Turn) -> i64 {
        match turn {
            Turn::Left => -1,
            Turn::Right => 1,
        }
    }
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    fn from_int(turn: i64) -> Turn {
        match turn {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Cannot create Turn from {}.", turn),
        }
    }
}

pub struct PaintRobot {
    computer: InputOutputComputer,
    painted_fields: HashMap<Point, Color>,
    current_position: (Point, Direction),
}

impl PaintRobot {
    pub fn new(memory: Vec<i64>) -> PaintRobot {
        let computer = InputOutputComputer::new(memory);

        PaintRobot {
            computer,
            painted_fields: HashMap::new(),
            current_position: (Point(0, 0), Direction::Up),
        }
    }

    pub fn painted_area_to_string(&self) -> String {
        let Point(min_x, min_y) = self.fold_keys(isize::min);
        let Point(max_x, max_y) = self.fold_keys(isize::max);

        let mut result = String::with_capacity(((max_x - min_x) * (max_y - min_y)) as usize);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.painted_fields.get(&Point(x, y)).unwrap_or(&Color::Black) {
                    Color::Black => result += " ",
                    Color::White => result += "#",
                }
            }
            result += "\n";
        }

        result
    }

    fn fold_keys(&self, cmp: fn(isize, isize) -> isize) -> Point {
        self.painted_fields
            .keys()
            .fold(Point(0, 0), |Point(min_x, min_y), &Point(x, y)| {
                Point(cmp(min_x, x), cmp(min_y, y))
            })
    }

    pub fn get_num_at_least_once_painted_fields(&self) -> usize {
        self.painted_fields.len()
    }

    pub fn paint(&mut self, starting_color: Color) {
        self.painted_fields.insert(Point(0, 0), starting_color);
        while self.paint_current_field_and_move_to_next_field() {}
    }

    pub fn paint_current_field_and_move_to_next_field(&mut self) -> bool {
        let (position, direction) = self.current_position;
        let current_color = self.painted_fields.get(&position).unwrap_or(&Color::Black);

        // send current color
        self.computer.write_int(current_color.to_int() as i32);

        if let Some(new_color) = self.computer.read_int().unwrap() {
            let new_color = Color::from_int(new_color);
            self.painted_fields.insert(position, new_color);

            let turn = self
                .computer
                .read_int()
                .unwrap()
                .expect("Turn direction has not been outputted.");
            let new_direction = direction.turn(Turn::from_int(turn));
            let new_position = PaintRobot::move_forward(position, new_direction);
            self.current_position = (new_position, new_direction);

            true
        } else {
            false
        }
    }

    fn move_forward(position: Point, direction: Direction) -> Point {
        match direction {
            Direction::Up => position - Point(0, 1),
            Direction::Right => position + Point(1, 0),
            Direction::Down => position + Point(0, 1),
            Direction::Left => position - Point(1, 0),
        }
    }
}

pub struct InputOutputComputer {
    input_writer: ChannelWriter,
    output_reader: IntOutputReader<BufReader<ChannelReader>>,
    async_computer: AsyncIntComputer,
}

impl InputOutputComputer {
    pub fn new(memory: Vec<i64>) -> InputOutputComputer {
        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();
        let async_computer = AsyncIntComputer::new(memory, input_receiver, output_sender);

        let input_writer = ChannelWriter::new(input_sender);
        let output_reader =
            IntOutputReader::new(BufReader::new(ChannelReader::new(output_receiver)));

        InputOutputComputer {
            input_writer,
            output_reader,
            async_computer,
        }
    }

    pub fn read_int(&mut self) -> Result<Option<i64>, io::Error> {
        self.output_reader.read_int()
    }

    pub fn write_int(&self, input: i32) -> Result<(), SendError<i32>> {
        self.input_writer.write_int(input)
    }
}

pub struct IntOutputReader<T: BufRead> {
    reader: T,
}

impl<T: BufRead> IntOutputReader<T> {
    pub fn new(reader: T) -> IntOutputReader<T> {
        IntOutputReader { reader }
    }

    pub fn read_int(&mut self) -> Result<Option<i64>, io::Error> {
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;

            if bytes_read == 0 {
                return Ok(None);
            }

            let output_pattern = "Output value:";

            if let Some(idx) = line.find(output_pattern) {
                let part = &line[idx + output_pattern.len()..].trim();
                let result = i64::from_str(part)
                    .map_err(|err| io::Error::new(ErrorKind::InvalidInput, err))?;
                return Ok(Some(result));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
