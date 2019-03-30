use aoc13::read_street_map;

fn main() {
    let mut street_map = read_street_map("input.txt").unwrap();

    while street_map.check_collision().is_empty() {
        street_map.advance();
    }

    for point in street_map.check_collision().iter() {
        println!("{}", point);
    }
}