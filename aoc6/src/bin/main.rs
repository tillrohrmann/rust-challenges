use aoc6::create_patch_map;
use aoc6::create_distance_map;

fn main() {
    let coordinates = aoc6::read_coordinates("input.txt").unwrap();

    let map = create_patch_map(&coordinates);

    println!("{:?}", map.calculate_finite_patch_sizes().values().max());

    let distance_map = create_distance_map(&coordinates, 10000);

    println!("{:?}", distance_map.calculate_finite_patch_sizes().values());
}