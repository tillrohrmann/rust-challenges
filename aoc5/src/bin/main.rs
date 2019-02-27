fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = aoc5::react_polymer(input.trim());

    println!("{}", result);
    println!("{}: {}", aoc5::is_minimal(&result), result.len());

    let (min_chr, min_polymer) = aoc5::find_minimal_polymer(input.trim());

    println!("{}, {}", min_chr, min_polymer);
}