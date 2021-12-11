fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let result = aoc_2021_10::score_input(&content).unwrap();
    println!("Part one: {}", result);
}