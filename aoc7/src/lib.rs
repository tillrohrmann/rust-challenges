#[macro_use]
extern crate lazy_static;

use aoc_common;
use aoc_common::GenericResult;
use regex::Regex;
use aoc_common::GenericError;

#[derive(Debug, PartialEq)]
struct Dependency(char, char);

fn read_dependencies(path: &str) -> GenericResult<Vec<Dependency>> {
    let raw_input = aoc_common::read_raw_file_content(path)?;

    raw_input.iter().map(|input| parse_dependency(input)).collect()
}

fn parse_dependency(input: &str) -> GenericResult<Dependency> {
    lazy_static! {
        static ref LOG_REGEX: Regex = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    }

    LOG_REGEX
        .captures(input)
        .map(|captures| {
            internal_parse_dependency(&captures)
        })
        .ok_or(GenericError::new(&format!("Could not parse input: {}", input)).into())
        .and_then(|r| r)
}

fn internal_parse_dependency(captures: &regex::Captures) -> GenericResult<Dependency> {
    let source = parse_from_capture(captures, 1);
    let target = parse_from_capture(captures, 2);

    source.and_then(|s| target.map(|t| Dependency(s, t)))
}

fn parse_from_capture(captures: &regex::Captures, index: usize) -> GenericResult<char> {
    captures.get(index)
        .and_then(|m| m.as_str().chars().next())
        .ok_or(GenericError::new(&format!("Could not find dependency {}", index)).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency() {
        assert_eq!(parse_dependency("Step Z must be finished before step N can begin.").unwrap(), Dependency('Z', 'N'));
    }
}
