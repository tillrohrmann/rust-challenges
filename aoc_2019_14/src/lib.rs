use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fmt, io};

type Chemical = String;

#[derive(Debug, Clone)]
pub struct QuantifiedChemical {
    amount: usize,
    chemical: Chemical,
}

impl QuantifiedChemical {
    fn new(amount: usize, chemical: Chemical) -> QuantifiedChemical {
        QuantifiedChemical { chemical, amount }
    }
}

impl FromStr for QuantifiedChemical {
    type Err = ParseChemicalReactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split(" ").collect();

        if parts.len() != 2 {
            return Err(ParseChemicalReactionError());
        }

        let amount = parts[0].parse()?;
        let chemical = parts[1].into();

        Ok(QuantifiedChemical::new(amount, chemical))
    }
}

#[derive(Debug)]
pub struct ChemicalReaction {
    inputs: Vec<QuantifiedChemical>,
    output: QuantifiedChemical,
}

impl ChemicalReaction {
    fn new(inputs: Vec<QuantifiedChemical>, output: QuantifiedChemical) -> ChemicalReaction {
        ChemicalReaction { inputs, output }
    }
}

impl FromStr for ChemicalReaction {
    type Err = ParseChemicalReactionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arrow_parts: Vec<&str> = s.split("=>").collect();

        if arrow_parts.len() != 2 {
            return Err(ParseChemicalReactionError());
        }

        let inputs: Result<Vec<QuantifiedChemical>, ParseChemicalReactionError> = arrow_parts[0]
            .split(",")
            .map(|input| input.parse())
            .collect();
        let inputs = inputs?;
        let output = arrow_parts[1].parse()?;

        Ok(ChemicalReaction::new(inputs, output))
    }
}

#[derive(Debug)]
pub struct ParseChemicalReactionError();

impl fmt::Display for ParseChemicalReactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Could not parse chemical reaction.")
    }
}

impl std::error::Error for ParseChemicalReactionError {}

impl From<ParseIntError> for ParseChemicalReactionError {
    fn from(_: ParseIntError) -> Self {
        ParseChemicalReactionError()
    }
}

impl From<ParseChemicalReactionError> for io::Error {
    fn from(error: ParseChemicalReactionError) -> Self {
        io::Error::new(ErrorKind::InvalidInput, error)
    }
}

pub fn read_chemical_reactions_from_file(path: &str) -> io::Result<Vec<ChemicalReaction>> {
    parse_chemical_reactions(aoc_common::read_raw_file_content(path)?)
        .map_err(|err| io::Error::from(err))
}

fn parse_chemical_reactions(
    reactions: Vec<String>,
) -> Result<Vec<ChemicalReaction>, ParseChemicalReactionError> {
    reactions.into_iter().map(|line| line.parse()).collect()
}

pub struct ReactionResult {
    produced: usize,
    required: usize,
}

impl ReactionResult {
    pub fn required_constituent(&self) -> usize {
        self.required
    }
}

pub struct Reactor {}

impl Reactor {
    pub fn build_reactor(chemical_reactions: &Vec<ChemicalReaction>) -> Reactor {
        Reactor {}
    }

    pub fn calculate_reaction(
        &self,
        target: QuantifiedChemical,
    ) -> HashMap<Chemical, ReactionResult> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_fuel_creation() {
        let chemical_reactions = create_first_chemical_reactions();
        let expected_ore_constituent = 31;

        run_test(&chemical_reactions, expected_ore_constituent);
    }

    #[test]
    fn second_fuel_creation() {
        let chemical_reactions = create_second_chemical_reactions();
        let expected_ore_constituent = 165;

        run_test(&chemical_reactions, expected_ore_constituent);
    }

    #[test]
    fn third_fuel_creation() {
        let chemical_reactions = create_third_chemical_reactions();
        let expected_ore_constituent = 13312;

        run_test(&chemical_reactions, expected_ore_constituent);
    }

    #[test]
    fn fourth_fuel_creation() {
        let chemical_reactions = create_fourth_chemical_reactions();
        let expected_ore_constituent = 180697;

        run_test(&chemical_reactions, expected_ore_constituent);
    }

    #[test]
    fn fifth_fuel_creation() {
        let chemical_reactions = create_fifth_chemical_reactions();
        let expected_ore_constituent = 2210736;

        run_test(&chemical_reactions, expected_ore_constituent);
    }

    fn run_test(chemical_reactions: &Vec<ChemicalReaction>, expected_ore_constituent: usize) {
        let reactor = Reactor::build_reactor(&chemical_reactions);
        let reaction_result = reactor.calculate_reaction(QuantifiedChemical::new(1, "FUEL".into()));
        assert_eq!(
            reaction_result
                .get("ORE".into())
                .map(|reaction| reaction.required_constituent())
                .unwrap_or(0),
            expected_ore_constituent
        );
    }

    fn create_first_chemical_reactions() -> Vec<ChemicalReaction> {
        let input = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

        parse_raw_string_into_chemical_reactions(input)
    }

    fn create_second_chemical_reactions() -> Vec<ChemicalReaction> {
        let input = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

        parse_raw_string_into_chemical_reactions(input)
    }

    fn create_third_chemical_reactions() -> Vec<ChemicalReaction> {
        let input = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        parse_raw_string_into_chemical_reactions(input)
    }

    fn create_fourth_chemical_reactions() -> Vec<ChemicalReaction> {
        let input = r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        parse_raw_string_into_chemical_reactions(input)
    }

    fn create_fifth_chemical_reactions() -> Vec<ChemicalReaction> {
        let input = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

        parse_raw_string_into_chemical_reactions(input)
    }

    fn parse_raw_string_into_chemical_reactions(input: &str) -> Vec<ChemicalReaction> {
        let lines: Vec<String> = input.split("\n").map(|str| str.into()).collect();
        parse_chemical_reactions(lines).unwrap()
    }
}
