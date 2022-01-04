use aoc_2021_11::Simulator;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let mut simulator = Simulator::parse(&content).unwrap();

    simulator.simulate(100);
    let result = simulator.get_num_flashes();

    println!("Part one: {}", result);
}
