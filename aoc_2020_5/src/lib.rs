use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    pub fn new(row: usize, column: usize) -> BoardingPass {
        BoardingPass {
            row,
            column,
        }
    }

    fn parse_number(input: &str) -> usize {
        let mut result = 0;

        for char in input.chars() {
            result <<= 1;

            result |= match char {
                'B' | 'R' => 1,
                'F' | 'L' => 0,
                x => panic!("Unsupported")
            }
        }

        result
    }

    pub fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 10 {
            let (row, column) = s.split_at(7);

            let row = BoardingPass::parse_number(row);
            let column = BoardingPass::parse_number(column);

            Ok(BoardingPass::new(row, column))
        } else {
            Err(format!("Input {} does not contain 10 characters.", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BoardingPass;

    #[test]
    fn simple_boarding_pass() {
        let boarding_pass: BoardingPass = "BFFFBBFRRR".parse::<BoardingPass>().unwrap();
        assert_eq!(boarding_pass, BoardingPass::new(70, 7));
    }
}
