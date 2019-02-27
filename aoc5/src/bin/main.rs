fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = aoc5::react_polymer(&input);

    println!("{}", result);
    println!("{}", result.len());
}