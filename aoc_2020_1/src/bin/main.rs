fn main() {
    let input = aoc_2020_1::read_numbers("input.txt").unwrap();
    let result = aoc_2020_1::find_two_numbers(&input);

    println!("Result: {}", result.unwrap_or(-1));
}