use aoc_2021_12::Map;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();

    let map = Map::parse(&content).unwrap();
    part_one(&map);
    part_two(&map);
}

fn part_one(map: &Map) {
    let result = map.count_distinct_paths();
    println!("Part one result: {}", result);
}

fn part_two(map: &Map) {
    let result = map.count_distinct_paths_with_rep();
    println!("Part two result: {}", result);
}