use std::collections::HashSet;
use aoc_common::GenericResult;

enum ParseResult {
    Ok(Vec<char>),
    Error(char),
}

pub fn score_input(input: &Vec<String>) -> GenericResult<u32> {
    let result = input.iter().map(|line| score(line)).map(|result| {
        match result {
            ParseResult::Ok(_) => 0,
            ParseResult::Error(error) => get_value(error),
        }
    }).sum();

    Ok(result)
}

trait Element {
    fn is_opening(&self) -> bool;

    fn fit(&self, other: &Self) -> bool;
}

impl Element for char {

    fn is_opening(&self) -> bool {
        vec!['(', '[', '{', '<'].contains(self)
    }

    fn fit(&self, other: &Self) -> bool {
        match *self {
            '(' => ')' == *other,
            ')' => '(' == *other,
            '[' => ']' == *other,
            ']' => '[' == *other,
            '{' => '}' == *other,
            '}' => '{' == *other,
            '<' => '>' == *other,
            '>' => '<' == *other,
            _ => false,
        }
    }
}

fn score(line: &String) -> ParseResult {
    let mut stack = Vec::new();

    for next_char in line.chars() {
        if next_char.is_opening() {
            stack.push(next_char)
        } else {
            let popped_char = stack.pop().unwrap();

            if !popped_char.fit(&next_char) {
                return ParseResult::Error(next_char);
            }
        }
    }

    stack.reverse();
    ParseResult::Ok(stack)
}

fn get_value(ch: char) -> u32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn score_auto_completion(input: &Vec<String>) -> GenericResult<usize> {
    let mut result: Vec<usize> = input.iter().map(|line| score(line)).flat_map(|result| {
        match result {
            ParseResult::Ok(completion) => Some(score_completion(&completion)),
            ParseResult::Error(_) => None,
        }
    }).collect();

    result.sort();

    Ok(result[result.len()/2])
}

fn score_completion(completion: &Vec<char>) -> usize {
    let mut result = 0;

    for &ch in completion {
        result = result * 5 + value(ch);
    }

    result
}

fn value(ch: char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let input = input();
        assert_eq!(score_input(&input).unwrap(), 26397);
    }

    fn input() -> Vec<String> {
        let input: Vec<String> = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".split('\n').map(|str| str.to_string()).collect();
        input
    }

    #[test]
    fn test_auto_completion_scoring() {
        let input = input();
        assert_eq!(score_auto_completion(&input).unwrap(), 288957)
    }
}
