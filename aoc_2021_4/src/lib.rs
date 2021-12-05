use std::collections::HashSet;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BingoBoard {
    board: Vec<u32>,
    sum: u32,
}

impl BingoBoard {
    fn new(board: Vec<u32>) -> BingoBoard {
        assert_eq!(board.len(), 25);
        let sum = board.iter().sum();
        BingoBoard { board, sum }
    }

    fn parse_from_str(input: &[&str]) -> Result<BingoBoard, ParseIntError> {
        let joined_result: String = input.join(" ");

        let board = joined_result
            .split(" ")
            .filter(|word| !word.is_empty())
            .map(|word| word.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;

        Ok(BingoBoard::new(board))
    }

    fn is_winning(&self, numbers: &HashSet<u32>) -> bool {
        self.calculate_winning_score(numbers) > 0
    }

    fn calculate_winning_score(&self, numbers: &HashSet<u32>) -> u32 {
        let mut winning = false;
        let mut diff = 0;

        for row in 0..5 {
            let mut matches = 0;
            for column in 0..5 {
                let value = self.board[row * 5 + column];
                if numbers.contains(&value) {
                    matches += 1;
                    diff += value;
                }
            }

            if matches == 5 {
                winning = true
            }
        }

        for column in 0..5 {
            let mut matches = 0;
            for row in 0..5 {
                let value = self.board[row * 5 + column];
                if numbers.contains(&value) {
                    matches += 1;
                }
            }

            if matches == 5 {
                winning = true
            }
        }

        if winning {
            self.sum - diff
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct NumberDrawing {
    numbers: Vec<u32>,
}

impl NumberDrawing {
    fn new(numbers: Vec<u32>) -> NumberDrawing {
        NumberDrawing { numbers }
    }

    fn parse_from_str(input: &str) -> Result<NumberDrawing, ParseIntError> {
        let numbers = input
            .trim()
            .split(",")
            .map(|word| word.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;
        Ok(NumberDrawing::new(numbers))
    }
}

#[derive(Debug)]
struct Game {
    numberDrawing: NumberDrawing,
    boards: Vec<BingoBoard>,
}

impl Game {
    fn parse_from_input(input: &Vec<&str>) -> Result<Game, ParseIntError> {
        let numberDrawing = NumberDrawing::parse_from_str(input[0])?;

        let mut boards = Vec::new();

        for index in (2..input.len()).step_by(6) {
            boards.push(BingoBoard::parse_from_str(&input[index..(index + 6)])?);
        }

        Ok(Game {
            numberDrawing,
            boards,
        })
    }

    fn play(&self) -> u32 {
        let mut numbers = HashSet::new();

        for &number in &self.numberDrawing.numbers {
            numbers.insert(number);

            let winning_boards = self.find_winning_boards(&numbers);

            if !winning_boards.is_empty() {
                println!("{:?}", winning_boards);
                println!("Numbers: {:?}", numbers);
                assert_eq!(winning_boards.len(), 1);

                return number * winning_boards[0].calculate_winning_score(&numbers);
            }
        }

        0
    }

    fn find_last_board(&self) -> u32 {
        let mut numbers = HashSet::new();
        let mut boards: HashSet<&BingoBoard> = self.boards.iter().collect();

        for &number in &self.numberDrawing.numbers {
            if boards.is_empty() {
                break;
            }

            numbers.insert(number);

            let winning_boards = boards
                .iter()
                .cloned()
                .filter(|board| board.is_winning(&numbers))
                .collect::<Vec<&BingoBoard>>();

            boards.retain(|board| !board.is_winning(&numbers));

            if boards.is_empty() {
                println!("{:?}", winning_boards);
                println!("Numbers: {:?}", numbers);
                assert_eq!(winning_boards.len(), 1);

                return number * winning_boards[0].calculate_winning_score(&numbers);
            }
        }

        0
    }

    fn find_winning_boards(&self, numbers: &HashSet<u32>) -> Vec<&BingoBoard> {
        self.boards
            .iter()
            .filter(|board| board.is_winning(numbers))
            .collect()
    }
}

pub fn play_game(input: &Vec<&str>) -> Result<u32, ParseIntError> {
    let game = Game::parse_from_input(input)?;

    Ok(game.play())
}

pub fn play_losing_game(input: &Vec<&str>) -> Result<u32, ParseIntError> {
    let game = Game::parse_from_input(input)?;

    Ok(game.find_last_board())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_bingo_board() {
        let input = vec![
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
        ];

        BingoBoard::parse_from_str(&input).unwrap();
    }
}
