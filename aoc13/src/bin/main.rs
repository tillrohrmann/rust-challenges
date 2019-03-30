use aoc13::{read_street_map, StreetMap};

fn main() {
    let mut street_map = read_street_map("input.txt").unwrap();

//    solve_part_one(&mut street_map)
    solve_part_two(&mut street_map);

}

fn solve_part_one(street_map: &mut StreetMap) -> () {
    while street_map.check_collision().is_empty() {
        street_map.advance(false);
        println!("{}", street_map);
    }

    for point in street_map.check_collision().iter() {
        println!("{}", point);
    }
}

fn solve_part_two(street_map: &mut StreetMap) -> () {
    while street_map.get_cars().len() > 1 {
        street_map.advance(true);
    }

    for car in street_map.get_cars().iter() {
        println!("{}", car.get_position());
    }
}