use aoc_2019_10::Map;
use aoc_common::math::Point;

fn main() {
    let map = Map::load_from_file("input.txt").unwrap();
    println!("{:?}", map.find_best_asteroid());

    let mut asteroid_iterator = map.vaporize_asteroids(Point(30, 34));

    let mut asteroid_iterator = asteroid_iterator.skip(199);

    println!("{:?}", asteroid_iterator.next());
}
