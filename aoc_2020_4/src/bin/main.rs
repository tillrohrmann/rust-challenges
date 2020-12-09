use aoc_2020_4::{Validator, Passport};

fn main() {
    let passports = aoc_2020_4::parse_passports("input.txt").unwrap();
    println!("#Passports: {}", passports.len());
    solve_part_1(&passports);
    solve_part_2(&passports);
}

fn solve_part_1(passports: &Vec<Passport>) {
    let validator = aoc_2020_4::SimpleValidator::new();
    let num_valid_passports = passports.iter().filter(|passport| validator.validate_passport(passport).unwrap()).count();

    println!("Part 1: {}", num_valid_passports);
}

fn solve_part_2(passports: &Vec<Passport>) {
    let validator = aoc_2020_4::Part2Validator::new();
    let num_valid_passports = passports.iter().filter(|passport| validator.validate_passport(passport).unwrap()).count();

    println!("Part 2: {}", num_valid_passports);
}