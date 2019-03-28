use aoc_common::{GenericResult, GenericError, read_raw_file_content};
use aoc_common::collections::RadixTree;
use bit_vec::BitVec;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum GrowingResult {
    Growing,
    NotGrowing,
}

#[derive(PartialEq, Debug, Clone)]
pub struct GrowingRule(BitVec, GrowingResult);

impl GrowingRule {
    const GROWING_SYMBOL: char = '#';

    pub fn parse(rule: &str) -> GenericResult<GrowingRule> {
        let splits: Vec<&str> = rule.split(" => ").collect();

        if splits.len() == 2 {
            let condition = convert_to_bit_vec(splits[0]);
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

fn convert_to_bit_vec(condition: &str) -> BitVec {
    let mut result = BitVec::from_elem(condition.len(), false);

    for (index, chr) in condition.chars().enumerate() {
        if chr == GrowingRule::GROWING_SYMBOL {
            result.set(index, true);
        }
    }

    result
}

pub struct GrowingRules {
    growing_rules: HashSet<BitVec>,
}

impl GrowingRules {
    pub fn new(growing_rules: &Vec<GrowingRule>) -> GrowingRules {
        GrowingRules{
            growing_rules: growing_rules.iter().filter(|&GrowingRule(_, growing_result)| *growing_result == GrowingResult::Growing).map(|GrowingRule(condition, _)| condition).cloned().collect()
        }
    }

    pub fn grows(&self, condition: BitVec) -> bool {
        self.growing_rules.contains(&condition)
    }
}

pub struct PlantPots {
    configuration: BitVec,
    offset: isize,
    growing_rules: GrowingRules,
}

impl PlantPots {
    pub fn new(initial_configuration: BitVec, growing_rules: &Vec<GrowingRule>) -> PlantPots {
        PlantPots {
            configuration: initial_configuration,
            offset: 0,
            growing_rules: GrowingRules::new(growing_rules),
        }
    }

    pub fn advance(&mut self) {
        let left_border = self.grow_left_border();
        let right_border = self.grow_right_border();

        let length = self.configuration.len();
        let mut new_length = length;

        if left_border {
            new_length += 1;
            self.offset += 1;
        }

        if right_border {
            new_length += 1;
        }

        let mut new_configuration = BitVec::from_elem(new_length, false);

        let offset = if left_border {
            new_configuration.set(0, true);
            1
        } else {
            0
        };

        if right_border {
            new_configuration.set(new_length - 1, true);
        }

        for (index, _) in (0..(length)).map(|index| (index, self.growing_rules.grows(PlantPots::extract_slice(&self.configuration, index as isize - 2, 5)))).filter(|&(_, grows)| grows == true) {
            new_configuration.set(index + offset, true);
        }

        let zeros = PlantPots::count_starting_zeros(&new_configuration);

        if zeros >= 1 {
            let mut z_configuration = BitVec::from_elem(new_configuration.len() - zeros, false);

            for (index, value) in new_configuration.iter().skip(zeros).enumerate() {
                z_configuration.set(index, value);
            }

            self.configuration = z_configuration;
            self.offset -= zeros as isize;
        } else {
            self.configuration = new_configuration;
        }
    }

    fn count_starting_zeros(configuration: &BitVec) -> usize {
        configuration.iter().take_while(|&value| !value).count()
    }

    fn grow_left_border(&self) -> bool {
        let next = PlantPots::extract_slice(&self.configuration, - 3, 5);
        self.growing_rules.grows(next)
    }

    fn grow_right_border(&self) -> bool {
        let next = PlantPots::extract_slice(&self.configuration, self.configuration.len() as isize - 2, 5);
        self.growing_rules.grows(next)
    }

    fn extract_slice(configuration: &BitVec, start: isize, length: usize) -> BitVec {
        let mut result = BitVec::from_elem(length, false);

        let start_index = std::cmp::max(0, - start) as usize; // start_index + start >= 0
        let end_index = std::cmp::min(5, configuration.len() as isize - start) as usize; // end_index + start <= configuration.len()

        for index in start_index..end_index {
            result.set(index, configuration.get((index as isize + start) as usize).unwrap());
        }

        result
    }

    pub fn get_configuration(&self) -> &BitVec {
        &self.configuration
    }

    pub fn count_plants(&self) -> usize {
        self.configuration.iter().filter(|&plant| plant).count()
    }

    pub fn sum_plant_containing_pots(&self) -> isize {
        self.configuration.iter().enumerate().filter(|&(_, chr)| chr).map(|(index, _)| index as isize - self.offset as isize).sum::<isize>()
    }

    pub fn get_growing_rules(&self) -> &GrowingRules {
        &self.growing_rules
    }

    pub fn get_offset(&self) -> isize {
        self.offset
    }
}

pub fn parse_plant_pots_file(path: &str) -> GenericResult<PlantPots> {
    let file_content = read_raw_file_content(path)?;

    let initial_configuration_line = file_content.get(0).ok_or(GenericError::new("File was empty."))?;

    if initial_configuration_line.find("initial state: ").is_none() {
        Err(GenericError::new("File does not start with initial state line.").into())
    } else {
        let initial_configuration = &initial_configuration_line[15..];

        let mut configuration = BitVec::from_elem(initial_configuration.len(), false);
        for (index, chr) in initial_configuration.chars().enumerate() {
            if chr == GrowingRule::GROWING_SYMBOL {
                configuration.set(index, true);
            }
        }

        let growing_rules: Vec<GrowingRule> = file_content.iter().skip(2).map(|line| GrowingRule::parse(line)).collect::<GenericResult<Vec<GrowingRule>>>()?;

        Ok(PlantPots::new(configuration, &growing_rules))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_growing_rule() {
        assert_eq!(GrowingRule::parse(".#... => #").unwrap(), GrowingRule(convert_to_bit_vec(".#..."), GrowingResult::Growing))
    }

    #[test]
    fn test_parse_plant_pots_file() {
        let plant_pots = parse_plant_pots_file("test_input.txt").unwrap();

        assert_eq!(plant_pots.get_configuration(), &convert_to_bit_vec("#..#.#..##......###...###"));

        let growing_rules = plant_pots.get_growing_rules();

        assert_eq!(growing_rules.grows(convert_to_bit_vec(".#.##")), true);
        assert_eq!(growing_rules.grows(convert_to_bit_vec("#....")), false);
    }

    #[test]
    fn test_plant_pots_evolution() {
        let mut plant_pots = parse_plant_pots_file("test_input.txt").unwrap();

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration(), &convert_to_bit_vec("#...#....#.....#..#..#..#"));

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration(), &convert_to_bit_vec("##..##...##....#..#..#..##"));
    }
}
