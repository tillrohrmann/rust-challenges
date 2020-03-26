use aoc_2019_14::QuantifiedChemical;

fn main() -> () {
    let chemical_reactions = aoc_2019_14::read_chemical_reactions_from_file("input.txt").unwrap();
    let reactor = aoc_2019_14::Reactor::build_reactor(&chemical_reactions);
    let reactor_result = reactor.calculate_reaction(QuantifiedChemical::new(1, "FUEL".to_owned()));

    println!(
        "Chemical reactions: {:?}",
        reactor_result
            .constituent("ORE".into())
            .map(|r| r.required_constituent())
            .unwrap_or(0)
    );
}
