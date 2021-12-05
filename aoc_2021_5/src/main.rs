fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    solve_part_one(&content);
}

fn solve_part_one(content: &Vec<String>) {
    let result = aoc_2021_5::calculate_overlapping_points(&content).unwrap();
    println!("Part one: {}", result);
}
