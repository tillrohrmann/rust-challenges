use aoc_common::math::Point;
use aoc_2020_3::Map;

fn main() {
    let map = aoc_2020_3::Map::parse_from_file("input.txt").unwrap();

    solve_part_1(&map);
    solve_part_2(&map);
}

fn solve_part_1(map: &Map) {
    let result = aoc_2020_3::solve_slope(Point(3, 1), &map);

    println!("{}", result);
}

fn solve_part_2(map: &Map) {
    let slopes = vec![Point(1, 1), Point(3, 1), Point(5, 1), Point(7, 1), Point(1, 2)];

    let result = slopes.into_iter().map(|slope| aoc_2020_3::solve_slope(slope, &map)).fold(1, |a, b| a * b);

    println!("{}", result);
}