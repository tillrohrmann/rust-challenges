fn main() {
    let input = aoc_2020_1::read_numbers("input.txt").unwrap();
    let part_one = solve_part_one(&input);
    let part_two = solve_part_two(&input);

    println!("Part one: {}", part_one.unwrap_or(-1));
    println!("Part two: {}", part_two.unwrap_or(-1));
}

fn solve_part_one(input: &Vec<i32>) -> Result<i32, String> {
    let result = aoc_2020_1::find_two_numbers(input);
    result
}

fn solve_part_two(input: &Vec<i32>) -> Result<i32, String> {
    aoc_2020_1::find_three_numbers(input)
}