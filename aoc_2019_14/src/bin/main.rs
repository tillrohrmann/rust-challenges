use aoc_2019_14::{QuantifiedChemical, FuelCalculator};

fn main() -> () {
    let chemical_reactions = aoc_2019_14::read_chemical_reactions_from_file("input.txt").unwrap();
    let reactor = aoc_2019_14::Reactor::build_reactor(&chemical_reactions);
    solve_part_1(&reactor);
    solve_part_2(reactor);
}

fn solve_part_1(reactor: &aoc_2019_14::Reactor) {
    let reactor_result = reactor.calculate_reaction(QuantifiedChemical::new(1, "FUEL".to_owned()));
    println!(
        "Chemical reactions: {:?}",
        reactor_result
            .constituent("ORE".into())
            .map(|r| r.required_constituent())
            .unwrap_or(0)
    );
}

fn solve_part_2(reactor: aoc_2019_14::Reactor) {
    let fuel_calculator = FuelCalculator::new(reactor);
    let ore = 1_000_000_000_000;
    let max_fuel = fuel_calculator.calculate_max_fuel(ore);

    println!("Max fuel with {} ore: {}", ore, max_fuel);
}
