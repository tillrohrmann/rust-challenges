use std::time::Instant;
use euler_68::*;

fn main() {
    let input = (1..=10).collect();
    let permutate = Permutate::new(&input);

    let start = Instant::now();

    let result = permutate
        .filter(|permutation| is_valid_configuration(permutation))
        .map(|valid_permutation| concatenate_permutation(&valid_permutation))
        .filter(|number| count_digits(*number) == 16)
        .max();

    println!("Time elapsed {:?}.", start.elapsed());

    println!("Result {}", result.unwrap());
}