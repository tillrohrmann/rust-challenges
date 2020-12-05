#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;
use std::fmt::Formatter;
use std::error::Error;
use std::io::{BufReader, BufRead};

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    BufReader::new(file).lines().collect()
}

pub enum ValidationMode {
    Count,
    Position,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PasswordLine {
    policy: Policy,
    password: String,
}

impl PasswordLine {
    pub fn new(policy: Policy, password: String) -> PasswordLine {
        PasswordLine {
            policy,
            password,
        }
    }

    pub fn is_valid(&self, validation_mode: ValidationMode) -> bool {
        match validation_mode {
            ValidationMode::Count => {
                let mut character_counts = HashMap::new();

                for char in self.password.chars() {
                    let counter = character_counts.entry(char).or_insert(0);
                    *counter += 1;
                }

                self.policy.is_compliant_with_count(&character_counts)
            },
            ValidationMode::Position => {
                self.policy.is_compliant_with_position(&self.password)
            }
        }
    }
}

impl FromStr for PasswordLine {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(":");

        let policy: Policy = splits.next().ok_or("Could not find policy.")?.trim().parse()?;
        let password = splits.next().ok_or("Could not find password.")?.trim();

        Ok(PasswordLine::new(policy, password.to_string()))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Policy {
    characters: HashMap<char, Range>,
}

impl Policy {
    pub fn new(characters: HashMap<char, Range>) -> Policy {
        Policy {
            characters,
        }
    }

    fn is_compliant_with_position(&self, password: &String) -> bool {
        self.characters.iter().all(|(key, range)| Policy::complies_to(password, key, range))
    }

    fn complies_to(password: &String, key: &char, range: &Range) -> bool {
        let mut chars = password.chars();

        let first_char = chars.nth((range.min - 1) as usize);
        let second_char = chars.nth((range.max - range.min - 1) as usize);

        if let (Some(first), Some(second)) = (first_char, second_char) {
            let result = (first.eq(key)) ^ (second.eq(key));
            result
        } else {
            false
        }
    }

    fn is_compliant_with_count(&self, counts: &HashMap<char, i32>) -> bool {
        self.characters.iter().all(|(key, range)| range.is_in_range(
            counts
                .get(key)
                .map(|&x| x)
                .unwrap_or(0)))
    }
}

impl FromStr for Policy {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(' ');
        let range: Range = splits.next().ok_or("Could not find range.")?.trim().parse()?;
        let characters = splits.next().ok_or("Could not find characters.")?.trim();

        let mut policies = HashMap::new();

        for char in characters.chars() {
            policies.insert(char, range);
        }

        Ok(Policy::new(policies))
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn new(min: i32, max: i32) -> Range {
        Range {
            min,
            max,
        }
    }

    fn is_in_range(&self, count: i32) -> bool {
        self.min <= count && count <= self.max
    }
}

impl FromStr for Range {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d*)-(\d*)").unwrap();
        }

        if let Some(captures) = RE.captures(s) {
            let min = captures[1].parse()?;
            let max = captures[2].parse()?;

            Ok(Range::new(min, max))
        } else {
            Err(format!("Could not parse Range from {}.", s).into())
        }
    }
}

pub type GenericError = Box<dyn std::error::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password_line_position() {
        let password_line = "1-3 a: abcde".parse::<PasswordLine>().unwrap();

        assert!(password_line.is_valid(ValidationMode::Position));
    }

    #[test]
    fn test_is_invalid_password_line_position() {
        let password_line = "1-3 b: cdefg".parse::<PasswordLine>().unwrap();

        assert!(!password_line.is_valid(ValidationMode::Position));
    }

    #[test]
    fn test_is_invalid_password_line_position_2() {
        let password_line = "2-9 c: ccccccccc".parse::<PasswordLine>().unwrap();

        assert!(!password_line.is_valid(ValidationMode::Position));
    }

    #[test]
    fn test_is_valid_password_line() {
        let password_line = "1-7 c: foobarc".parse::<PasswordLine>().unwrap();

        assert_eq!(password_line.is_valid(ValidationMode::Count), true);
    }

    #[test]
    fn test_is_invalid_password_line() {
        let password_line = "1-7 d: foobarc".parse::<PasswordLine>().unwrap();

        assert_eq!(password_line.is_valid(ValidationMode::Count), false);
    }

    #[test]
    fn test_password_line_parsing() {
        let password_line: PasswordLine = "1-7 c: foobarc".parse().unwrap();

        assert_eq!(password_line, PasswordLine::new(createPolicy(), "foobarc".to_string()));
    }

    #[test]
    fn test_policy_parsing() {
        let result: Policy = "1-7 c".parse().unwrap();
        let policy = createPolicy();

        assert_eq!(result, policy)
    }

    fn createPolicy() -> Policy {
        let mut characters = HashMap::new();
        characters.insert('c', Range::new(1, 7));
        Policy::new(characters)
    }

    #[test]
    fn test_range_parsing() {
        let result: Range = "1-7".parse().unwrap();

        assert_eq!(result, Range::new(1, 7));
    }
}
