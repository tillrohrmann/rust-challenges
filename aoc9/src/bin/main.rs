use aoc9::MarbleGame;

fn main() {
    let game = MarbleGame::new(458, 72019);
    let result = game.play_game();

    println!("{}", result);
}