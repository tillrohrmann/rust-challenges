use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file.");

    let lines = input.trim().split("\n");

    let mut total_fuel = 0;

    for line in lines {
        let mass = i32::from_str(line).expect("Could not parse integer.");
        let fuel = compute_correct_fuel(mass);
        total_fuel += fuel;
    }

    println!("Total fuel {}", total_fuel);
}

fn compute_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn compute_correct_fuel(mass: i32) -> i32 {
    let required_fuel = compute_fuel(mass);

    if required_fuel <= 0 {
        0
    } else {
        required_fuel + compute_correct_fuel(required_fuel)
    }
}
