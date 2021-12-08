use aoc_common::GenericResult;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let initial_positions = parse_initial_positions(&content).unwrap();

    solve_part_one(&initial_positions);
    solve_part_two(&initial_positions);
}

fn solve_part_one(initial_positions: &Vec<u32>) {
    let result = aoc_2021_7::calculate_minimal_fuel(&initial_positions);
    println!("Part one: {}.", result);
}

fn solve_part_two(initial_positions: &Vec<u32>) {
    let result = aoc_2021_7::calculate_minimal_fuel_with_increasing_rate(&initial_positions);
    println!("Part two: {}.", result);
}

fn parse_initial_positions(content: &Vec<String>) -> GenericResult<Vec<u32>> {
    content
        .iter()
        .flat_map(|line| line.split(','))
        .map(|number| number.parse::<u32>())
        .collect::<Result<_, _>>()
        .map_err(|err| err.into())
}
