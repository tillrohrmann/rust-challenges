use aoc_2020_5::BoardingPass;

fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();
    let boarding_passes = input.into_iter()
        .map(|line| line.parse::<BoardingPass>())
        .collect::<Result<Vec<BoardingPass>, String>>()
        .unwrap();

    let result = boarding_passes.iter().map(|boarding_pass| boarding_pass.seat_id()).max();

    println!("Result {}", result.unwrap())
}