use aoc_2020_2::{PasswordLine, ValidationMode};

fn main() {
    let password_lines = read_password_lines();

    solve_part_1(&password_lines);
    solve_part_2(&password_lines);
}

fn solve_part_1(password_lines: &Vec<PasswordLine>) {
    let result = password_lines.iter()
        .filter(|password_line| password_line.is_valid(ValidationMode::Count))
        .count();

    println!("{}", result);
}

fn solve_part_2(password_lines: &Vec<PasswordLine>) {
    let result = password_lines.iter()
        .filter(|password_line| password_line.is_valid(ValidationMode::Position))
        .count();

    println!("{}", result);
}

fn read_password_lines() -> Vec<PasswordLine> {
    let lines = aoc_2020_2::read_lines("input.txt").unwrap();

    let password_lines: Vec<PasswordLine> = lines
        .into_iter()
        .map(|line| line.parse::<aoc_2020_2::PasswordLine>())
        .collect::<Result<Vec<PasswordLine>, aoc_2020_2::GenericError>>().unwrap();
    password_lines
}