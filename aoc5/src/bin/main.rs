fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = aoc5::react_polymer(input.trim());

    println!("{}", result);
    println!("{}: {}", aoc5::is_minimal(&result), result.len());
}