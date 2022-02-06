use aoc_common::GenericResult;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Rule([char; 2], char);

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("->");

        let condition = splits
            .next()
            .ok_or("Required condition.".to_string())?
            .trim();
        let result = splits.next().ok_or("Required result.".to_string())?.trim();

        if condition.len() != 2 {
            return Err("Condition needs two chars.".to_string());
        }

        if result.len() != 1 {
            return Err("Result needs one chars.".to_string());
        }

        let mut chars = condition.chars();
        let condition = [chars.next().unwrap(), chars.next().unwrap()];

        Ok(Rule(condition, result.chars().next().unwrap()))
    }
}

#[derive(Debug)]
pub struct PolymerRules {
    rules: HashMap<[char; 2], char>,
}

impl PolymerRules {
    fn parse(input: &[String]) -> GenericResult<PolymerRules> {
        let rules = input
            .iter()
            .map(|line| line.parse().map(|Rule(key, value)| (key, value)))
            .collect::<Result<_, _>>()?;

        Ok(PolymerRules { rules })
    }

    fn get(&self, key: &[char]) -> Option<&char> {
        self.rules.get(key)
    }
}

#[derive(Debug)]
pub struct Polymer {
    polymer: Vec<char>,
}

impl Polymer {
    pub fn count_elements(&self) -> HashMap<char, usize> {
        self.polymer
            .iter()
            .fold(HashMap::new(), |mut acc, element| {
                *acc.entry(*element).or_insert(0) += 1;
                acc
            })
    }

    fn new(input: &String) -> Polymer {
        Polymer {
            polymer: input.chars().collect(),
        }
    }
}

pub fn parse_input(content: &Vec<String>) -> GenericResult<(Polymer, PolymerRules)> {
    let polymer = Polymer::new(content.get(0).ok_or("No polymer line available.")?);
    let rules = PolymerRules::parse(&content[2..])?;

    Ok((polymer, rules))
}

pub fn evolve(polymer: &Polymer, rules: &PolymerRules, steps: usize) -> Option<Polymer> {
    let mut current_polymer = None;

    for _ in 0..steps {
        current_polymer = Some(evolve_step(
            current_polymer.as_ref().unwrap_or(polymer),
            rules,
        ));
    }

    current_polymer
}

fn evolve_step(polymer: &Polymer, rules: &PolymerRules) -> Polymer {
    let mut result = Vec::new();

    for i in 0..polymer.polymer.len() - 1 {
        result.push(polymer.polymer[i]);
        if let Some(&insert) = rules.get(&polymer.polymer[i..=(i + 1)]) {
            result.push(insert);
        }
    }

    result.push(polymer.polymer[polymer.polymer.len() - 1]);

    Polymer { polymer: result }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
