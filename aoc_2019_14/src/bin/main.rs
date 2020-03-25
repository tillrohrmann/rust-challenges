fn main() -> () {
    let chemical_reactions = aoc_2019_14::read_chemical_reactions_from_file("input.txt").unwrap();

    println!("Chemical reactions: {:?}", chemical_reactions);
}
