use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    find_first_frequency_which_appears_twice();
}

fn frequency_vector(path: &str) -> Vec<i32> {
    let input = std::fs::read_to_string("input.txt").unwrap();

    input.split_whitespace().map(|freq| i32::from_str(freq).unwrap()).collect()
}

fn calculate_initial_frequency() {
    let result = frequency_vector("input.txt").iter().sum::<i32>();

    println!("Result {}.", result);
}

fn find_first_frequency_which_appears_twice() {
    let frequency_vector = frequency_vector("input.txt");
    let mut frequency_cache = HashSet::with_capacity(frequency_vector.len());
    let mut frequency = 0;
    let mut freq_iter = frequency_vector.iter();

    while !frequency_cache.contains(&frequency) {
        frequency_cache.insert(frequency);

        match freq_iter.next() {
            Some(freq) => frequency += freq,
            None => {
                freq_iter = frequency_vector.iter();
                frequency += freq_iter.next().unwrap();
            }
        }
    }

    println!("Twice frequency: {}", frequency);
}
