use aoc_2021_12::Map;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let map = Map::parse(&content).unwrap();
    let result = map.count_distinct_paths();
    println!("Part one result: {}", result);
}