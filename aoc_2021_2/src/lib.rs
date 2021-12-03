use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let command = parts.next().ok_or("First part is missing.".to_string())?;
        let length = parts
            .next()
            .ok_or("Second part is missing.".to_string())?
            .parse::<i32>()
            .map_err(|err| "Could not parse length.".to_string())?;

        match command {
            "forward" => Ok(Command::Forward(length)),
            "up" => Ok(Command::Up(length)),
            "down" => Ok(Command::Down(length)),
            _ => Err("Could not parse command.".to_string()),

        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn zero() -> Position {
        Position { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<Command> for Position {
    fn from(command: Command) -> Self {
        match command {
            Command::Forward(x) => Position::new(x, 0),
            Command::Up(x) => Position::new(0, -x),
            Command::Down(x) => Position::new(0, x),
        }
    }
}

fn calculate_position(commands: &Vec<Command>) -> Position {
    let mut position = Position::zero();

    for &command in commands {
        position += command.into();
    }

    position
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn simulate_submarine(input: &Vec<&str>) -> Result<Position, String> {
    let commands = parse_commands(input)?;

    Ok(calculate_position(&commands))
}

fn parse_commands(input: &Vec<&str>) -> Result<Vec<Command>, String> {
    input.iter().map(|line| line.parse()).collect()
}

pub fn simulate_submarine_aim(input: &Vec<&str>) -> Result<Position, String> {
    let commands = parse_commands(input)?;

    Ok(calculate_position_aim(&commands))
}

fn calculate_position_aim(commands: &Vec<Command>) -> Position {
    let mut position = Position::zero();
    let mut aim = 0;

    for &command in commands {
        match command {
            Command::Forward(x) => position += Position::new(x, aim * x),
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }

    position
}