use aoc_2019_10::Map;

fn main() {
    let map = Map::load_from_file("input.txt").unwrap();

    println!("{:?}", map.find_best_asteroid());
}
