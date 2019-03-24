use aoc_common::read_raw_file_content;
use aoc10::{parse_points_with_velocity, PointMap, InitialComponents};

fn main() {
    let raw_file_content = read_raw_file_content("input.txt").unwrap();
    let points_with_velocity = parse_points_with_velocity(&raw_file_content).unwrap();

    let mut display_map = PointMap::new(points_with_velocity);

    println!("Size: {:?}", display_map.size());

    let number_points = display_map.number_points();
    let mut counter = 0;
    let mut min_connected_components = usize::max_value();
    let mut min_connected_components_counter = 0;

    while display_map.number_points() == number_points {
        let connected_components = InitialComponents::new(
            display_map.points_iter().collect(),
            display_map.size()).calculate_connected_components();

        if min_connected_components > connected_components.number_connected_components() {
            min_connected_components = connected_components.number_connected_components();
            min_connected_components_counter = counter;
        }

        if connected_components.number_connected_components() < 100 {
            display_map.display();
            println!();
            println!();
        }

        display_map.advance();
        counter += 1;
    }

    println!("{}", counter);
    println!("Min CCs: {}, counter: {}", min_connected_components, min_connected_components_counter);

//    for i in 0..=1000 {
//        println!("Time: {}", i);
//        let connected_components = InitialComponents::new(
//            display_map.points_iter().collect(),
//            display_map.size()).calculate_connected_components();
//
//        println!("CC: {}", connected_components.number_connected_components());
//
//        display_map.advance();
//    }
}