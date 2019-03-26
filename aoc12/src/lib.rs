use aoc_common::{GenericResult, GenericError, read_raw_file_content};
use aoc_common::collections::RadixTree;

#[derive(PartialEq, Debug, Clone)]
enum GrowingResult {
    Growing,
    NotGrowing,
}

#[derive(PartialEq, Debug, Clone)]
pub struct GrowingRule(String, GrowingResult);

impl GrowingRule {
    const GROWING_SYMBOL: char = '#';

    pub fn parse(rule: &str) -> GenericResult<GrowingRule> {
        let splits: Vec<&str> = rule.split(" => ").collect();

        if splits.len() == 2 {
            let condition = splits[0].to_string();
            let conclusion = splits[1];

            if conclusion.len() != 1 {
                Err(GenericError::new(&format!("Conclusion {} could not be parsed.", conclusion)).into())
            } else {
                let growing_result = if conclusion == &GrowingRule::GROWING_SYMBOL.to_string() {
                    GrowingResult::Growing
                } else {
                    GrowingResult::NotGrowing
                };

                Ok(GrowingRule(condition, growing_result))
            }
        } else {
            Err(GenericError::new(&format!("Input {} cannot be parsed into GrowingRule.", rule)).into())
        }
    }
}

pub struct GrowingRules {
    growing_rules: Vec<GrowingRule>,
}

impl GrowingRules {
    pub fn new(growing_rules: &Vec<GrowingRule>) -> GrowingRules {
        GrowingRules{
            growing_rules: growing_rules.iter().filter(|&GrowingRule(_, growing_result)| *growing_result == GrowingResult::Growing).cloned().collect()
        }
    }

    pub fn grows(&self, condition: &str) -> bool {
        self.growing_rules.contains(&GrowingRule(condition.to_string(), GrowingResult::Growing))
    }
}

pub struct PlantPots {
    configuration: String,
    offset: usize,
    growing_rules: GrowingRules,
}

impl PlantPots {
    pub fn new(initial_configuration: &str, offset: usize, growing_rules: &Vec<GrowingRule>) -> PlantPots {
        PlantPots {
            configuration: initial_configuration.to_string(),
            offset,
            growing_rules: GrowingRules::new(growing_rules),
        }
    }

    pub fn advance(&mut self) {
        let length = self.configuration.len();
        let mut new_configuration = vec!['.'; length];

        for (index, _) in (2..(length - 2)).map(|index| (index, self.growing_rules.grows(&self.configuration[index-2..=index+2]))).filter(|&(_, grows)| grows == true) {
            new_configuration[index] = GrowingRule::GROWING_SYMBOL;
        }

        self.configuration = new_configuration.iter().collect();
    }

    pub fn get_configuration(&self) -> &str {
        &self.configuration
    }

    pub fn count_plants(&self) -> usize {
        self.configuration.chars().filter(|&chr| chr == GrowingRule::GROWING_SYMBOL).count()
    }

    pub fn sum_plant_containing_pots(&self) -> isize {
        self.configuration.chars().enumerate().filter(|&(_, chr)| chr == GrowingRule::GROWING_SYMBOL).map(|(index, _)| index as isize - self.offset as isize).sum::<isize>()
    }

    pub fn get_growing_rules(&self) -> &GrowingRules {
        &self.growing_rules
    }
}

pub fn parse_plant_pots_file(path: &str, additional_space: usize) -> GenericResult<PlantPots> {
    let file_content = read_raw_file_content(path)?;

    let initial_configuration_line = file_content.get(0).ok_or(GenericError::new("File was empty."))?;

    if initial_configuration_line.find("initial state: ").is_none() {
        Err(GenericError::new("File does not start with initial state line.").into())
    } else {
        let initial_configuration = &initial_configuration_line[15..];

        let prefix: String = vec!['.'; additional_space].iter().collect::<String>();

        let growing_rules: Vec<GrowingRule> = file_content.iter().skip(2).map(|line| GrowingRule::parse(line)).collect::<GenericResult<Vec<GrowingRule>>>()?;

        Ok(PlantPots::new(&(prefix.clone() + initial_configuration + &prefix), additional_space, &growing_rules))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_growing_rule() {
        assert_eq!(GrowingRule::parse(".#... => #").unwrap(), GrowingRule(".#...".to_string(), GrowingResult::Growing))
    }

    #[test]
    fn test_parse_plant_pots_file() {
        let plant_pots = parse_plant_pots_file("test_input.txt", 20).unwrap();

        assert_eq!(plant_pots.get_configuration().contains("#..#.#..##......###...###"), true);

        let growing_rules = plant_pots.get_growing_rules();

        assert_eq!(growing_rules.grows(".#.##"), true);
        assert_eq!(growing_rules.grows("#...."), false);
    }

    #[test]
    fn test_plant_pots_evolution() {
        let mut plant_pots = parse_plant_pots_file("test_input.txt", 20).unwrap();

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration().contains("...#...#....#.....#..#..#..#..........."), true);

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration().contains("...##..##...##....#..#..#..##.........."), true);
    }
}
