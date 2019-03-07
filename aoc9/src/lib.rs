struct MarbleGame {}

impl MarbleGame {
    pub fn new() -> MarbleGame {
        MarbleGame{}
    }

    pub fn play_game(num_players: usize, last_marble: usize) -> usize {
        42
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game_1() {
        assert_eq!(MarbleGame::play_game(9, 25), 32);
    }

    #[test]
    fn test_play_game_2() {
        assert_eq!(MarbleGame::play_game(10, 1618), 8317);
    }

    #[test]
    fn test_play_game_3() {
        assert_eq!(MarbleGame::play_game(13, 7999), 146373);
    }

    #[test]
    fn test_play_game_4() {
        assert_eq!(MarbleGame::play_game(17, 1104), 2764);
    }

    #[test]
    fn test_play_game_5() {
        assert_eq!(MarbleGame::play_game(21, 6111), 54718);
    }

    #[test]
    fn test_play_game_6() {
        assert_eq!(MarbleGame::play_game(30, 5807), 37305);
    }
}
