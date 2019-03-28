use aoc12::parse_plant_pots_file;

fn main() {
    let mut plant_pots = parse_plant_pots_file("input.txt").unwrap();
    let steps = 50_000;

    for i in 0..100_000 {
        println!("{}: length {}, offset {}, count {}, sum {}", i * steps, plant_pots.get_configuration().len(), plant_pots.get_offset(), plant_pots.count_plants(), plant_pots.sum_plant_containing_pots());
        println!("{:?}", plant_pots.get_configuration());
        for _ in 0..steps {
            plant_pots.advance()
        }
    }

    println!("{:?}", plant_pots.get_configuration());
    println!("{}", plant_pots.sum_plant_containing_pots());
}