fn main() {
    let valid_passwords = aoc_2019_4::valid_passwords_in_range(138241, 674034, false);
    let valid_passwords_exact_doubles = aoc_2019_4::valid_passwords_in_range(138241, 674034, true);

    println!("Valid passwords: {}, valid passwords with exact doubles: {}", valid_passwords, valid_passwords_exact_doubles);
}
