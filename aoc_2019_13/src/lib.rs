use aoc_2019_2::IntComputer;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::io::{BufRead, BufReader, Error, ErrorKind, Stdout};
use std::rc::Rc;
use std::str::FromStr;

pub struct Pinball {
    game_memory: Vec<i64>,
}

impl Pinball {
    pub fn new(path: &str) -> Pinball {
        let mut game_memory = aoc_2019_2::read_memory_from_file(path);
        game_memory[0] = 2;
        Pinball { game_memory }
    }

    pub fn start(&self) {
        let mut game = Rc::new(RefCell::new(PinballGame::new()));
        let game_output_reader = PinballGameOutputReader::new(Rc::clone(&game));
        let joystick_controller = JoystickController::new(Rc::clone(&game));
        let mut output_reader = IntComputerOutputReader::new(Box::new(game_output_reader));
        let input = std::io::BufReader::new(std::io::stdin());

        let mut computer = IntComputer::new(
            self.game_memory.clone(),
            BufReader::new(joystick_controller),
            &mut output_reader,
        );

        computer.compute();

        game.borrow_mut().finalize_input_sequence();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

impl core::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let output = match self {
            Tile::EMPTY => " ",
            Tile::WALL => "#",
            Tile::BLOCK => "B",
            Tile::PADDLE => "_",
            Tile::BALL => "*",
        };

        write!(f, "{}", output)
    }
}

impl From<isize> for Tile {
    fn from(value: isize) -> Self {
        match value {
            0 => Tile::EMPTY,
            1 => Tile::WALL,
            2 => Tile::BLOCK,
            3 => Tile::PADDLE,
            4 => Tile::BALL,
            _ => std::panic!("Unknown tile type {}", value),
        }
    }
}

struct Display {
    display: Vec<Vec<Tile>>,
    score: isize,
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        Display {
            display: vec![vec![Tile::EMPTY; width]; height],
            score: 0,
        }
    }

    fn draw(&self) -> () {
        for line in &self.display {
            for tile in line {
                print!("{}", tile);
            }
            println!();
        }

        println!("Score: {}", self.score);
    }

    fn update_tile(&mut self, point: aoc_common::math::Point, tile: Tile) -> () {
        let aoc_common::math::Point(x, y) = point;

        self.display[y as usize][x as usize] = tile;
    }

    fn update_score(&mut self, score_value: isize) -> () {
        self.score = score_value;
    }

    fn find_first(&self, tile_to_find: Tile) -> Option<aoc_common::math::Point> {
        for (y, line) in self.display.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if *tile == tile_to_find {
                    return Some(aoc_common::math::Point(x as isize, y as isize));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
enum GameElement {
    SCORE(isize),
    TILE(aoc_common::math::Point, Tile),
}

struct PinballGame {
    display: Display,
    next_joystick_move: Option<Joystick>,
}

impl PinballGame {
    fn new() -> PinballGame {
        PinballGame {
            display: Display::new(44, 23),
            next_joystick_move: None,
        }
    }

    fn notify_display(&mut self, game_element: GameElement) {
        match game_element {
            GameElement::SCORE(score_value) => self.display.update_score(score_value),
            GameElement::TILE(point, tile) => self.display.update_tile(point, tile),
        }
    }

    fn finalize_input_sequence(&mut self) {
        self.display.draw();
        self.calculate_next_joystick_move();
    }

    fn calculate_next_joystick_move(&mut self) {
        let ball = self.display.find_first(Tile::BALL);
        let paddle = self.display.find_first(Tile::PADDLE);

        self.next_joystick_move = Some(match (ball, paddle) {
            (
                Some(aoc_common::math::Point(x_ball, _)),
                Some(aoc_common::math::Point(x_paddle, _)),
            ) => match x_ball.cmp(&x_paddle) {
                std::cmp::Ordering::Equal => Joystick::NEUTRAL,
                std::cmp::Ordering::Less => Joystick::LEFT,
                std::cmp::Ordering::Greater => Joystick::RIGHT,
            },
            _ => Joystick::NEUTRAL,
        })
    }
}

struct PinballGameOutputReader {
    buffer: Vec<isize>,
    game: Rc<RefCell<PinballGame>>,
}

impl PinballGameOutputReader {
    fn new(game: Rc<RefCell<PinballGame>>) -> PinballGameOutputReader {
        PinballGameOutputReader {
            buffer: Vec::with_capacity(16),
            game,
        }
    }
}

impl OutputReader for PinballGameOutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        let value =
            isize::from_str(output_value).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        self.buffer.push(value);

        while self.buffer.len() >= 3 {
            let x = self.buffer[0];
            let y = self.buffer[1];
            let score_value = (-1, 0);

            let game_element = if (x, y) == score_value {
                GameElement::SCORE(self.buffer[2])
            } else {
                GameElement::TILE(aoc_common::math::Point(x, y), Tile::from(self.buffer[2]))
            };

            self.game.borrow_mut().notify_display(game_element);

            self.buffer.drain(0..3);
        }

        Ok(())
    }

    fn finalize_input_sequence(&self) {
        self.game.borrow_mut().finalize_input_sequence();
    }
}

enum Joystick {
    LEFT,
    RIGHT,
    NEUTRAL,
}

pub struct StdoutOutputReader {}

impl StdoutOutputReader {
    pub fn new() -> StdoutOutputReader {
        StdoutOutputReader {}
    }
}

impl OutputReader for StdoutOutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        println!("{}", output_value);
        Ok(())
    }

    fn finalize_input_sequence(&self) {}
}

impl InputWriter for StdoutOutputReader {
    fn request_input(&self) -> Result<(), Error> {
        println!("Request input");
        Ok(())
    }
}

pub trait OutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error>;

    fn finalize_input_sequence(&self);
}

trait InputWriter {
    fn request_input(&self) -> Result<(), Error>;
}

pub struct IntComputerOutputReader {
    buffer: Vec<u8>,
    output_reader: Box<dyn OutputReader>,
}

impl IntComputerOutputReader {
    pub fn new(output_reader: Box<dyn OutputReader>) -> IntComputerOutputReader {
        IntComputerOutputReader {
            buffer: Vec::with_capacity(1024),
            output_reader,
        }
    }
}

impl std::io::Write for IntComputerOutputReader {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let terminates_input = buf.contains(&b'\n');
        self.buffer.extend_from_slice(buf);

        if terminates_input {
            let mut reader = BufReader::new(&self.buffer[..]);
            let mut line = String::new();

            let mut total_bytes_read = 0;
            loop {
                let bytes_read = reader.read_line(&mut line)?;
                total_bytes_read += bytes_read;

                if bytes_read > 0 {
                    if line.starts_with(aoc_2019_2::OUTPUT_PREFIX) {
                        let output_value =
                            line.trim_start_matches(aoc_2019_2::OUTPUT_PREFIX).trim();
                        self.output_reader.read(output_value)?
                    } else {
                        self.output_reader.finalize_input_sequence()
                    }
                } else {
                    break;
                }
            }

            self.buffer.drain(0..total_bytes_read);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}

struct JoystickController {
    game: Rc<RefCell<PinballGame>>,
}

impl JoystickController {
    fn new(game: Rc<RefCell<PinballGame>>) -> JoystickController {
        JoystickController { game }
    }

    fn create_command(joystick_move: Joystick) -> String {
        match joystick_move {
            Joystick::NEUTRAL => "0\n",
            Joystick::LEFT => "-1\n",
            Joystick::RIGHT => "1\n",
        }
        .into()
    }
}

impl std::io::Read for JoystickController {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let joystick_move = self.game.borrow_mut().next_joystick_move.take();

        let joystick_move = joystick_move.unwrap();

        let command = JoystickController::create_command(joystick_move);

        command.as_bytes().read(buf)
    }
}
