fn main() {
    let passports = aoc_2020_4::parse_passports("input.txt").unwrap();
    let num_valid_passports = passports.iter().filter(|passport| passport.is_valid()).count();

    println!("Part 1: {}", num_valid_passports);
}