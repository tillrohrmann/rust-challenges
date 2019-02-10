use aoc2::*;

fn main() {
    let input = read_file_lines("input.txt");

    let result = input
        .iter()
        .map(|line| count_letters(line))
        .map(|(two, three)| {
            let two = if two > 0 {
                1
            } else {
                0
            };

            let three = if three > 0 {
                1
            } else {
                0
            };
            (two, three)
        })
        .fold((0, 0), |(acc_two, acc_three), (two, three)| (acc_two + two, acc_three + three));

    println!("Result {:?}", result.0 * result.1)
}