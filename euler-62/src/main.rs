use std::collections::HashMap;

fn main() {
    let number_cubes = 5;
    let mut cache = HashMap::new();

    for i in 1..10000 {
        let normal_form = sort_digits(cube(i));

        let entries = cache
            .entry(normal_form)
            .or_insert_with(|| Vec::with_capacity(1));

        entries.push(i);

        if entries.len() == number_cubes {
            println!("Cubes: {:?}", entries);
            println!("Normal form {}, smallest {}", normal_form, cube(entries[0]));

            break;
        }
    }
}

fn cube(n: u64) -> u64 {
    n * n * n
}

fn sort_digits(n: u64) -> u64 {
    let mut digits: Vec<u8> = vec![0; 10];

    let mut number = n;

    while number > 0 {
        let digit = (number % 10) as usize;
        number /= 10;

        digits[digit] += 1;
    }

    let mut result: u64 = 0;

    for i in (0..digits.len()).rev() {
        for j in 0..digits[i] {
            result *= 10;
            result += i as u64;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_digits() {
        assert_eq!(sort_digits(8421748509191), 9988754421110);
    }
}