use aoc2::*;

fn main() {
    let input = read_file_lines("input.txt");

    let result = calculate_checksum(&input);

    let identical_ids = find_identical_ids(input.iter().map(AsRef::as_ref).collect());

    println!("Result {:?}", result.0 * result.1);

    println!("Identical ids: {:?}", identical_ids);
}