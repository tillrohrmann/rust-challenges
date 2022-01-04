use aoc_2021_11::Simulator;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    solve_part_one(&content);
    solve_part_two(&content);
}

fn solve_part_two(content: &Vec<String>) {
    let mut simulator = Simulator::parse(&content).unwrap();

    let result = simulator.find_first_step_all_flash();

    println!("Part two: {}", result);
}

fn solve_part_one(content: &Vec<String>) {
    let mut simulator = Simulator::parse(&content).unwrap();

    simulator.simulate(100);
    let result = simulator.get_num_flashes();

    println!("Part one: {}", result);
}
