use std::fs;
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        assert_eq!(count_letters("abcccd"), (0, 1));
    }

    #[test]
    fn test_count_letters_2() {
        assert_eq!(count_letters("aabcdd"), (2, 0));
    }
}
