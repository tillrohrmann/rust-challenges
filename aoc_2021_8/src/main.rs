fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    solve_part_one(&content);
    solve_part_two(&content);
}

fn solve_part_two(content: &Vec<String>) {
    let result = aoc_2021_8::decode_results(&content).unwrap();
    println!("Part two: {}.", result);
}

fn solve_part_one(content: &Vec<String>) {
    let result = aoc_2021_8::find_1_4_7_8_digits(&content).unwrap();
    println!("Part one: {}.", result);
}
