use aoc_2019_2::IntComputer;
use std::io::{Error, ErrorKind, BufReader, BufRead, Stdout};
use std::str::FromStr;
use std::fmt::Formatter;

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
        let mut output_reader = IntComputerOutputReader::new();
        let input = std::io::BufReader::new(std::io::stdin());

        let mut computer = IntComputer::new(self.game_memory.clone(), input, &mut output_reader);

        computer.compute();
    }
}

#[derive(Debug, Copy, Clone)]
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

impl Into<String> for Tile {
    fn into(self) -> String {
        match self {
            Tile::EMPTY => " ",
            Tile::WALL => "#",
            Tile::BLOCK => "B",
            Tile::PADDLE => "--1",
            Tile::BALL => "*",
        }.into()
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
}

#[derive(Debug)]
enum GameElement {
    SCORE(isize),
    TILE(aoc_common::math::Point, Tile),
}

struct GameElementReader {
    buffer: Vec<isize>,
    display: Display,
}

impl GameElementReader {
    fn new() -> GameElementReader {
        GameElementReader {
            buffer: Vec::with_capacity(12),
            display: Display::new(44, 23),
        }
    }

    fn notify_display(&mut self, game_element: GameElement) {
        match game_element {
            GameElement::SCORE(score_value) => self.display.update_score(score_value),
            GameElement::TILE(point, tile) => self.display.update_tile(point, tile),
        }

        self.display.draw();
    }
}

impl OutputReader for GameElementReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        let value = isize::from_str(output_value).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

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

            self.notify_display(game_element);

            self.buffer.drain(0..3);
        }

        Ok(())
    }
}

struct StdoutOutputReader {}

impl StdoutOutputReader {
    fn new() -> StdoutOutputReader {
        StdoutOutputReader{}
    }
}

impl OutputReader for StdoutOutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error> {
        println!("{}", output_value);
        Ok(())
    }
}

trait OutputReader {
    fn read(&mut self, output_value: &str) -> Result<(), Error>;
}

struct IntComputerOutputReader {
    buffer: Vec<u8>,
    output_reader: Box<dyn OutputReader>,
}

impl IntComputerOutputReader {
    fn new() -> IntComputerOutputReader {
        IntComputerOutputReader {
            buffer: Vec::with_capacity(1024),
            output_reader: Box::new(GameElementReader::new()),
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
                        let output_value = line.trim_start_matches(aoc_2019_2::OUTPUT_PREFIX).trim();
                        self.output_reader.read(output_value)?
                    } else {
                        println!("{}", line);
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


