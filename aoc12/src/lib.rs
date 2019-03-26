use aoc_common::GenericResult;
use aoc_common::collections::RadixTree;

#[derive(PartialEq, Debug)]
enum GrowingResult {
    Growing,
    NotGrowing,
}

#[derive(PartialEq, Debug)]
pub struct GrowingRule(String, GrowingResult);

impl GrowingRule {
    pub fn parse(rule: &str) -> GrowingRule {
        unimplemented!()
    }
}

pub struct PlantPots {

}

impl PlantPots {
    pub fn new(initial_configuration: &str, growing_rules: &Vec<GrowingRule>) -> PlantPots {
        unimplemented!()
    }

    pub fn advance(&mut self) {

    }

    pub fn get_configuration(&self) -> &str {
        unimplemented!()
    }

    pub fn count_plants(&self) -> usize {
        unimplemented!()
    }

    pub fn grows(&self, configuration: &str) -> bool {
        unimplemented!()
    }
}

pub fn parse_plant_pots_file(path: &str) -> GenericResult<PlantPots> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_growing_rule() {
        assert_eq!(GrowingRule::parse(".#... => #"), GrowingRule(".#...".to_string(), GrowingResult::Growing))
    }

    #[test]
    fn test_parse_plant_pots_file() {
        let plant_pots = parse_plant_pots_file("test_input.txt").unwrap();

        assert_eq!(plant_pots.get_configuration().ends_with("#..#.#..##......###...###"), true);

        assert_eq!(plant_pots.grows(".#.##"), true);
        assert_eq!(plant_pots.grows("#...."), false);
    }

    #[test]
    fn test_plant_pots_evolution() {
        let mut plant_pots = parse_plant_pots_file("test_input.txt").unwrap();

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration().ends_with("...#...#....#.....#..#..#..#..........."), true);

        plant_pots.advance();
        assert_eq!(plant_pots.get_configuration().ends_with("...##..##...##....#..#..#..##.........."), true);
    }
}
