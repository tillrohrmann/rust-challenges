use aoc_2020_2::PasswordLine;

fn main() {
    let lines = aoc_2020_2::read_lines("input.txt").unwrap();

    let password_lines: Vec<PasswordLine> = lines
        .into_iter()
        .map(|line| line.parse::<aoc_2020_2::PasswordLine>())
        .collect::<Result<Vec<PasswordLine>, aoc_2020_2::GenericError>>().unwrap();

    let result = password_lines.iter().filter(|password_line| password_line.is_valid()).count();

    println!("{}", result);
}