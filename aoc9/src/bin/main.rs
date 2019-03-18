use aoc9::MarbleGame;

fn main() {
    let game = MarbleGame::new(458, 72019);
    let result = game.play_game();
    println!("{}", result);

    let large_game = MarbleGame::new(458, 72019 * 100);
    let large_result = large_game.play_game();
    println!("{}", large_result);
}