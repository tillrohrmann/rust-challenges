use aoc_2020_5::BoardingPass;

fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();
    let boarding_passes = input.into_iter()
        .map(|line| line.parse::<BoardingPass>())
        .collect::<Result<Vec<BoardingPass>, String>>()
        .unwrap();

    let mut seat_ids: Vec<usize> = boarding_passes.iter().map(|boarding_pass| boarding_pass.seat_id()).collect();

    let result = seat_ids.iter().max().unwrap().to_owned();

    seat_ids.sort();

    let mut id = 0;

    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i + 1] - seat_ids[i] > 1 {
            id = seat_ids[i] + 1;
        }
    }

    println!("Result part 1 {}", result);
    println!("Result part 2 {}", id);
}