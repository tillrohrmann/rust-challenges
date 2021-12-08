use aoc_common::GenericResult;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::str::FromStr;

struct Line {
    patterns: Vec<String>,
    results: Vec<String>,
}

impl Line {
    fn parse_values(values: &str) -> Vec<String> {
        values
            .trim()
            .split(' ')
            .map(|result| {
                let mut chars: Vec<char> = result.chars().collect();
                chars.sort_by(|a, b| a.cmp(b));

                String::from_iter(chars)
            })
            .collect()
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split('|');

        let patterns = splits.next().ok_or("Could not find patterns.")?;
        let results = splits.next().ok_or("Could not find results.")?;

        let patterns = Line::parse_values(patterns);
        let results = Line::parse_values(results);

        Ok(Line { patterns, results })
    }
}

struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &Vec<String>) -> GenericResult<Input> {
        let lines = input
            .iter()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(Input { lines })
    }
}

struct Decoder<'a> {
    line: &'a Line,
    encodings: HashMap<u8, &'a str>,
    rev_encodings: HashMap<&'a str, u8>,
}

impl<'a> Decoder<'a> {
    fn decode(line: &'a Line) -> Decoder {
        let mut encodings = HashMap::new();

        let mut one_encoding = None;
        let mut four_encoding = None;
        let mut seven_encoding = None;
        let mut eight_encoding = None;

        for pattern in &line.patterns {
            if pattern.len() == 2 {
                one_encoding = Some(pattern.as_str());
            }

            if pattern.len() == 4 {
                four_encoding = Some(pattern.as_str());
            }

            if pattern.len() == 3 {
                seven_encoding = Some(pattern.as_str());
            }

            if pattern.len() == 7 {
                eight_encoding = Some(pattern.as_str());
            }
        }

        encodings.insert(1, one_encoding.unwrap());
        encodings.insert(4, four_encoding.unwrap());
        encodings.insert(7, seven_encoding.unwrap());
        encodings.insert(8, eight_encoding.unwrap());

        let one_pattern: HashSet<char> = one_encoding.unwrap().chars().collect();
        let four_pattern: HashSet<char> = four_encoding.unwrap().chars().collect();

        let zero_six_or_nine = line.patterns.iter().filter(|pattern| pattern.len() == 6);

        let mut six_encoding = None;
        let mut nine_encoding = None;
        let mut zero_encoding = None;

        for candidate in zero_six_or_nine {
            let pattern: HashSet<char> = candidate.chars().collect();

            if one_pattern.is_subset(&pattern) {
                if four_pattern.is_subset(&pattern) {
                    nine_encoding = Some(candidate.as_str());
                } else {
                    zero_encoding = Some(candidate.as_str());
                }
            } else {
                six_encoding = Some(candidate.as_str());
            }
        }

        encodings.insert(0, zero_encoding.unwrap());
        encodings.insert(6, six_encoding.unwrap());
        encodings.insert(9, nine_encoding.unwrap());

        let nine_pattern: HashSet<char> = nine_encoding.unwrap().chars().collect();

        let mut two_encoding = None;
        let mut three_encoding = None;
        let mut five_encoding = None;

        for pattern in line.patterns.iter().filter(|pattern| pattern.len() == 5) {
            let unknown_pattern: HashSet<char> = pattern.chars().collect();

            if one_pattern.is_subset(&unknown_pattern) {
                three_encoding = Some(pattern.as_str());
            } else {
                if unknown_pattern.is_subset(&nine_pattern) {
                    five_encoding = Some(pattern.as_str());
                } else {
                    two_encoding = Some(pattern.as_str());
                }
            }
        }

        encodings.insert(2, two_encoding.unwrap());
        encodings.insert(3, three_encoding.unwrap());
        encodings.insert(5, five_encoding.unwrap());

        let rev_encodings: HashMap<&'a str, u8> = encodings.iter().map(|(&a, &b)| (b, a)).collect();

        Decoder {
            line,
            encodings,
            rev_encodings,
        }
    }

    fn find(&self, digit: u8) -> u32 {
        if let Some(&encoding) = self.encodings.get(&digit) {
            self.line
                .results
                .iter()
                .filter(|&result| result == encoding)
                .count() as u32
        } else {
            0
        }
    }

    fn decode_result(&self) -> u32 {
        let mut result: u32 = 0;
        for digit in &self.line.results {
            result = result * 10 + *self.rev_encodings.get(digit.deref()).unwrap() as u32;
        }

        result
    }
}

pub fn find_1_4_7_8_digits(input: &Vec<String>) -> GenericResult<u32> {
    let input = Input::parse(input)?;

    let mut result = 0;

    for line in &input.lines {
        let decoder = Decoder::decode(line);
        result += decoder.find(1) + decoder.find(4) + decoder.find(7) + decoder.find(8);
    }

    Ok(result)
}

pub fn decode_results(input: &Vec<String>) -> GenericResult<u32> {
    let input = Input::parse(input)?;

    let mut result = 0;

    for line in &input.lines {
        let decoder = Decoder::decode(line);
        result += decoder.decode_result();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = get_input();
        assert_eq!(find_1_4_7_8_digits(&input).unwrap(), 26);
    }

    #[test]
    fn test_decode_results() {
        let input = get_input();
        assert_eq!(decode_results(&input).unwrap(), 61229);
    }

    fn get_input() -> Vec<String> {
        vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
             "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
             "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
             "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
             "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
             "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
             "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
             "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
             "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
             "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ]
    }
}
