use aoc_common::collections::CircularLinkedList;

pub struct MarbleGame {
    num_players: usize,
    last_marble: usize,
}

impl MarbleGame {
    pub fn new(num_players: usize, last_marble: usize) -> MarbleGame {
        MarbleGame{
            num_players,
            last_marble,
        }
    }

    pub fn play_game(&self) -> usize {
        let mut marbles = vec![0].into_iter().collect::<CircularLinkedList<usize>>();

        let mut marble_cursor = marbles.cursor_mut();
        let mut current_player = 0;
        let mut next_marble = 1;
        let mut points = vec![0; self.num_players];

        while next_marble <= self.last_marble {
            if next_marble % 23 == 0 {
                points[current_player] += next_marble;

                for _ in 0..7 {
                    marble_cursor.current();
                    marble_cursor.move_back();
                }

                points[current_player] += marble_cursor.remove().unwrap();
            } else {
                marble_cursor.move_next();
                marble_cursor.current();
                marble_cursor.insert(next_marble);
                marble_cursor.move_next();
            }

            next_marble += 1;
            current_player = (current_player + 1) % self.num_players;
        }

        *points.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game_1() {
        assert_eq!(MarbleGame::new(9, 25).play_game(), 32);
    }

    #[test]
    fn test_play_game_2() {
        assert_eq!(MarbleGame::new(10, 1618).play_game(), 8317);
    }

    #[test]
    fn test_play_game_3() {
        assert_eq!(MarbleGame::new(13, 7999).play_game(), 146373);
    }

    #[test]
    fn test_play_game_4() {
        assert_eq!(MarbleGame::new(17, 1104).play_game(), 2764);
    }

    #[test]
    fn test_play_game_5() {
        assert_eq!(MarbleGame::new(21, 6111).play_game(), 54718);
    }

    #[test]
    fn test_play_game_6() {
        assert_eq!(MarbleGame::new(30, 5807).play_game(), 37305);
    }
}
