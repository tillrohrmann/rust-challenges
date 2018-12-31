use std::fmt::Display;

fn main() {
    println!("{:?}", series(1, 1000, triangle));

    let functions = [triangle, square, pentagonal, hexagonal, heptagonal, octagonal];

//    let functions = [triangle, square, pentagonal];
    let ranges:Vec<(usize, usize)> = functions.iter().map(|&f| find_range(4, f)).collect();

    let materialized_ranges: Vec<Vec<usize>> = ranges.iter().zip(functions.iter())
        .map(|((start, end), function)| series(*start, *end, *function)).collect();

    let result = find_cyclic_numbers(&materialized_ranges).unwrap();

    println!("Result {:?}", result);

    let sum: usize = result.iter().sum();
    println!("Sum {}", sum);
}

fn find_cyclic_numbers(materialized_ranges: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let starting_range = &materialized_ranges[0];
    let mut ranges: Vec<&Vec<usize>> = Vec::with_capacity(materialized_ranges.len() - 1);

    for i in 1..materialized_ranges.len() {
        ranges.push(&materialized_ranges[i]);
    }

    for &number in starting_range {
        let result = helper_cyclic_numbers(&mut vec![number], &ranges);

        match result {
            Some(r) => return Some(r),
            None => ()
        }
    }

    None
}

fn helper_cyclic_numbers(numbers: &mut Vec<usize>, ranges: &Vec<&Vec<usize>>) -> Option<Vec<usize>> {
    println!("{:?}", numbers);

    if ranges.is_empty() {
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        if last % 100 == first / 100 {
            Some(numbers.clone())
        } else {
            None
        }
    } else {
        let &number = numbers.last().unwrap();

        let new_starting_point = (number % 100) * 100;

        for (i, &range) in ranges.iter().enumerate() {
            let cyclic_numbers: &[usize] = cyclic_numbers_in_range(new_starting_point, range);

            if !cyclic_numbers.is_empty() {
                let mut remaining_ranges = Vec::with_capacity(ranges.len());
                for j in 0..i {
                    remaining_ranges.push(ranges[j]);
                }

                for j in (i + 1)..ranges.len() {
                    remaining_ranges.push(ranges[j]);
                }

                for &cyclic_number in cyclic_numbers {
                    if !numbers.contains(&cyclic_number) {
                        numbers.push(cyclic_number);

                        let result = helper_cyclic_numbers(numbers, &remaining_ranges);

                        match result {
                            Some(r) => return Some(r),
                            None => ()
                        }

                        numbers.remove(numbers.len() - 1);
                    }
                }
            }
        }

        None
    }
}

fn cyclic_numbers_in_range(starting_point: usize, range: &Vec<usize>) -> &[usize] {
    let start_result = range.binary_search(&starting_point);

    let start_index = match start_result {
        Ok(idx) => idx,
        Err(idx) => idx
    };

    let end_result = range.binary_search(&(starting_point + 100));

    let end_index = match end_result {
        Ok(idx) => idx,
        Err(idx) => idx
    };

    &range[start_index..end_index]
}

fn find_range(digits: usize, f: fn(usize) -> usize) -> (usize, usize) {
    let mut start = 1;

    while count_digits(f(start)) < digits {
        start += 1;
    }

    let mut end = start + 1;

    while count_digits(f(end)) == digits {
        end += 1
    }

    (start, end)
}


const DIGITS:[usize; 5] = [10, 100, 1000, 10000, 100000];

fn count_digits(n: usize) -> usize {
    let mut i = 0;

    while DIGITS[i] < n {
        i += 1;
    }

    if i == DIGITS.len() {
        panic!("Number {} is too large.", n);
    } else {
        i + 1
    }
}

fn series(start:usize, end: usize, f: fn(usize) -> usize) -> Vec<usize> {
    (start..=end).map(move |i| f(i)).collect()
}

fn triangle(n: usize) -> usize {
    n*(n + 1)/2
}

fn square(n: usize) -> usize {
    n*n
}

fn pentagonal(n: usize) -> usize {
    n*(3*n - 1)/2
}

fn hexagonal(n: usize) -> usize {
    n*(2*n -1)
}

fn heptagonal(n: usize) -> usize {
    n*(5*n - 3)/2
}

fn octagonal(n: usize) -> usize {
    n*(3*n - 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        let x = vec![1, 3, 5, 7, 9, 11];

        assert_eq!(x.binary_search(&4), Err(2));
    }
}
