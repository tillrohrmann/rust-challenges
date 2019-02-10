use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap();

    content.split_whitespace().map(|s| String::from(s)).collect()
}

pub fn count_letters(input: &str) -> (usize, usize) {
    let mut character_count = HashMap::with_capacity(26);
    for char in input.chars() {
        character_count.entry(char).and_modify(|v| *v += 1).or_insert(1);
    }

    let two_times= character_count.values().filter(|&&v| v == 2).count();
    let three_times = character_count.values().filter(|&&v| v == 3).count();

    (two_times, three_times)
}

pub fn calculate_checksum(input: &Vec<String>) -> (i32, i32) {
    let result = input
        .iter()
        .map(|line| count_letters(line))
        .map(|(two, three)| {
            let two = if two > 0 {
                1
            } else {
                0
            };

            let three = if three > 0 {
                1
            } else {
                0
            };
            (two, three)
        })
        .fold((0, 0), |(acc_two, acc_three), (two, three)| (acc_two + two, acc_three + three));
    result
}

pub fn find_identical_ids(input: Vec<&str>) -> HashSet<(String, String)> {
    input.iter()
        .flat_map(|&left| {
            input.iter().map(move |&right| {
                (left, right, calculate_distance(left, right))
            })
        })
        .filter(|(_, _, distance)| *distance == 1)
        .map(|(a, b, _)| {
            if a <= b {
                (String::from(a), String::from(b))
            } else {
                (String::from(b), String::from(a))
            }
        })
        .collect()
}

pub fn calculate_distance(a: &str, b: &str) -> usize {
    let result = a.chars()
        .zip(b.chars())
        .map(|(a_chr, b_chr)|
            if a_chr == b_chr {
                0
            } else {
                1
            })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_identical_ids() {
        let mut result_set = HashSet::with_capacity(1);
        result_set.insert((String::from("fghij"), String::from("fguij")));
        assert_eq!(find_identical_ids(vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]), result_set)
    }

    #[test]
    fn test_count_letters() {
        assert_eq!(count_letters("abcccd"), (0, 1));
    }

    #[test]
    fn test_count_letters_2() {
        assert_eq!(count_letters("aabcdd"), (2, 0));
    }

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance("till", "till"), 0);
    }

    #[test]
    fn test_calculate_distance_2() {
        assert_eq!(calculate_distance("tyll", "till"), 1);
    }
}
