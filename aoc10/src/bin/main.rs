use aoc_common::read_raw_file_content;
use aoc10::{parse_points_with_velocity, DisplayMap};

fn main() {
    let raw_file_content = read_raw_file_content("test_input.txt").unwrap();
    let points_with_velocity = parse_points_with_velocity(&raw_file_content).unwrap();

    let display_map = DisplayMap::new(points_with_velocity);

    display_map.display();
}