fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();

    let (paper, instructions) = aoc_2021_13::parse_input(&input).unwrap();
    let folded_paper = paper.fold(*instructions.iter().next().unwrap());
    println!("Result part one: {}.", folded_paper.count_points())
}
