use std::fs;
use std::num::ParseIntError;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.trim().split("\n");

    let numbers: Result<Vec<i32>, ParseIntError> = lines.map(|line| line.parse::<i32>()).collect();
    let numbers = numbers.unwrap();

    solve_part_one(&numbers);
    solve_part_two(&numbers);
}

fn solve_part_two(numbers: &Vec<i32>) {
    let mut previous: i32 = numbers[0..3].iter().sum();
    let mut result = 0;

    for i in 1..(numbers.len() - 2) {
        let next = numbers[i..(i + 3)].iter().sum();

        if previous < next {
            result += 1;
        }

        previous = next;
    }

    println!("Second part: {}", result);
}

fn solve_part_one(numbers: &Vec<i32>) {
    let number_increases = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("First part: {}", number_increases);
}
