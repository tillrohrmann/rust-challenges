fn main() {
    let lines = aoc_common::read_raw_file_content("input.txt").unwrap();
    solve_part_1(&lines);
    solve_part_2(&lines);
}

fn solve_part_2(lines: &Vec<String>) {
    let groups = aoc_2020_6::create_groups(lines);

    let group_votes: Vec<usize> = groups.iter().map(|group| group.count_unanimous_votes()).collect();

    let result: usize = group_votes.iter().sum();
    println!("Result part 2: {}", result);
}

fn solve_part_1(lines: &Vec<String>) {
    let grouped = aoc_2020_6::group_groups(&lines);

    let distinct_count: Vec<usize> = grouped.iter().map(|group| aoc_2020_6::count_distinct_votes(group)).collect();
    let result: usize = distinct_count.iter().sum();

    println!("{}", grouped.len());
    println!("{:?}", grouped);
    println!("{:?}", distinct_count);
    println!("Result part 1: {}", result);
}