use aoc12::parse_plant_pots_file;

fn main() {
    let mut plant_pots = parse_plant_pots_file("input.txt", 30).unwrap();

    for _ in 0..20 {
        plant_pots.advance();
    }

    println!("{}", plant_pots.sum_plant_containing_pots());
}