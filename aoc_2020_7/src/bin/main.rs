use aoc_2020_7::Bag;

fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();
    let bagGraph = aoc_2020_7::parse_bag_graph(&input);

    let part_1 = bagGraph.can_contain(&Bag("shiny gold".to_string()));

    println!("Result part 1: {}", part_1);
}