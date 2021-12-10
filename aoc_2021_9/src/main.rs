fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_two(input: &Vec<String>) {
    let result = aoc_2021_9::find_largest_basins(&input).unwrap();
    println!("Part two: {}.", result);
}

fn solve_part_one(input: &Vec<String>) {
    let result = aoc_2021_9::find_danger_points(&input).unwrap();
    println!("Part one: {}.", result);
}
