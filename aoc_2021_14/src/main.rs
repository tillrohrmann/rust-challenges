use std::cmp::Ordering;

fn main() {
    let content = aoc_common::read_raw_file_content("input.txt").unwrap();
    let (polymer, rules) = aoc_2021_14::parse_input(&content).unwrap();

    let evolved_polymer = aoc_2021_14::evolve(&polymer, &rules, 10).unwrap();
    let elements = evolved_polymer.count_elements();

    let min_element = elements.iter().min_by(compare_elements);
    let max_element = elements.iter().max_by(compare_elements);

    let result = max_element.map(|(_, &value)| value).unwrap_or(0)
        - min_element.map(|(_, &value)| value).unwrap_or(0);

    println!("Result for first part: {}.", result);
}

fn compare_elements(kv_one: &(&char, &usize), kv_two: &(&char, &usize)) -> Ordering {
    kv_one.1.cmp(kv_two.1)
}
