use std::collections::HashSet;
use aoc_common::GenericResult;

enum ParseResult {
    Ok,
    Error(char),
}

pub fn score_input(input: &Vec<String>) -> GenericResult<u32> {
    let result = input.iter().map(|line| score(line)).map(|result| {
        match result {
            ParseResult::Ok => 0,
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

    ParseResult::Ok
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
        assert_eq!(score_input(&input).unwrap(), 26397);
    }
}
