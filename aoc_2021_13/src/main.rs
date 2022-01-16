use aoc_2021_13::{FoldingInstruction, Paper};

fn main() {
    let input = aoc_common::read_raw_file_content("input.txt").unwrap();

    let (paper, instructions) = aoc_2021_13::parse_input(&input).unwrap();
    solve_part_one(&paper, &instructions);
    solve_part_two(&paper, &instructions);
}

fn solve_part_one(paper: &Paper, instructions: &Vec<FoldingInstruction>) {
    let folded_paper = paper.fold(instructions.iter().next().unwrap());
    println!("Result part one: {}.", folded_paper.count_points())
}

fn solve_part_two(initial_paper: &Paper, instructions: &Vec<FoldingInstruction>) {
    let mut resulting_paper: Option<Paper> = None;

    for instruction in instructions {
        resulting_paper = resulting_paper
            .as_ref()
            .or(Some(initial_paper))
            .map(|paper| paper.fold(instruction));
    }

    if let Some(result) = resulting_paper {
        println!("The result is:");
        println!("{}", result);
    }
}
