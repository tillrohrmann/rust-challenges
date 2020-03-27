use core::ops;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::ops::Mul;
use std::str::FromStr;
use std::{fmt, io};

type Chemical = String;

#[derive(Debug, Clone)]
pub struct QuantifiedChemical {
    amount: usize,
    chemical: Chemical,
}

impl QuantifiedChemical {
    pub fn new(amount: usize, chemical: Chemical) -> QuantifiedChemical {
        QuantifiedChemical { chemical, amount }
    }
}

impl ops::Mul<usize> for &QuantifiedChemical {
    type Output = QuantifiedChemical;

    fn mul(self, rhs: usize) -> Self::Output {
        QuantifiedChemical {
            amount: self.amount * rhs,
            chemical: self.chemical.clone(),
        }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct ReactionResult {
    produced: usize,
    required: usize,
}

impl ReactionResult {
    fn empty() -> ReactionResult {
        ReactionResult {
            produced: 0,
            required: 0,
        }
    }

    pub fn required_constituent(&self) -> usize {
        self.required
    }
}

pub struct Reactor {
    chemical_reactions: HashMap<Chemical, ChemicalReaction>,
}

impl Reactor {
    pub fn build_reactor(chemical_reactions: &Vec<ChemicalReaction>) -> Reactor {
        let index = chemical_reactions
            .iter()
            .map(|reaction| (reaction.output.chemical.clone(), reaction.clone()))
            .collect();
        Reactor {
            chemical_reactions: index,
        }
    }

    pub fn calculate_reaction(&self, target: QuantifiedChemical) -> ReactorResult {
        let mut reaction_results: HashMap<Chemical, ReactionResult> = HashMap::new();
        let mut required_chemicals = vec![target];

        while let Some(QuantifiedChemical { chemical, amount }) = required_chemicals.pop() {
            let reaction_result = reaction_results
                .entry(chemical.clone())
                .or_insert(ReactionResult::empty());

            if reaction_result.required + amount > reaction_result.produced {
                let chemical_reaction = self.chemical_reactions.get(&chemical);

                if let Some(chemical_reaction) = chemical_reaction {
                    let reaction_amount = chemical_reaction.output.amount;

                    let unused_produced_chemical =
                        reaction_result.produced - reaction_result.required;

                    let factor = ((amount - unused_produced_chemical) + (reaction_amount - 1))
                        / reaction_amount;

                    let produced_amount = factor * reaction_amount;

                    reaction_result.produced += produced_amount;

                    for input in &chemical_reaction.inputs {
                        required_chemicals.push(input * factor);
                    }
                } else {
                    // end of chemical reaction reached
                }
            }

            reaction_result.required += amount;
        }

        ReactorResult::new(reaction_results)
    }
}

#[derive(Debug)]
pub struct ReactorResult {
    reaction_results: HashMap<Chemical, ReactionResult>,
}

impl ReactorResult {
    fn new(reaction_results: HashMap<Chemical, ReactionResult>) -> ReactorResult {
        ReactorResult { reaction_results }
    }

    pub fn constituent(&self, chemical: Chemical) -> Option<ReactionResult> {
        self.reaction_results.get(&chemical).map(|r| r.clone())
    }
}

pub struct FuelCalculator {
    reactor: Reactor,
}

impl FuelCalculator {
    pub fn new(reactor: Reactor) -> FuelCalculator {
        FuelCalculator { reactor }
    }

    pub fn calculate_max_fuel(&self, ore: usize) -> usize {
        let mut fuel = 1;
        let mut lower_bound = None;
        let mut upper_bound = None;

        loop {
            if let (Some(lower), Some(upper)) = (lower_bound, upper_bound) {
                if upper - lower == 1 {
                    break;
                }
            }

            let reactor_result = self
                .reactor
                .calculate_reaction(QuantifiedChemical::new(fuel, "FUEL".into()));
            let required_ore = reactor_result
                .constituent("ORE".into())
                .map(|c| c.required_constituent())
                .unwrap_or(0);

            if required_ore <= ore {
                lower_bound = Some(fuel);
            } else {
                upper_bound = Some(fuel);
            }

            fuel = match (lower_bound, upper_bound) {
                (Some(lower), Some(upper)) => lower + (upper - lower) / 2,
                (None, Some(upper)) => upper / 2,
                (Some(lower), None) => lower * 2,
                _ => panic!("Unexpected bounds")
            }
        }

        lower_bound.unwrap_or(0)
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

    #[test]
    fn third_max_fuel_creation() {
        let chemical_reactions = create_third_chemical_reactions();
        let expected_fuel = 82892753;

        run_max_fuel_test(&chemical_reactions, expected_fuel);
    }

    #[test]
    fn fourth_max_fuel_creation() {
        let chemical_reactions = create_fourth_chemical_reactions();
        let expected_fuel = 5586022;

        run_max_fuel_test(&chemical_reactions, expected_fuel);
    }

    #[test]
    fn fifth_max_fuel_creation() {
        let chemical_reactions = create_fifth_chemical_reactions();
        let expected_fuel = 460664;

        run_max_fuel_test(&chemical_reactions, expected_fuel);
    }

    fn run_max_fuel_test(chemical_reactions: &Vec<ChemicalReaction>, expected_fuel: usize) {
        let reactor = Reactor::build_reactor(chemical_reactions);
        let fuel_calculator = FuelCalculator::new(reactor);

        let max_fuel = fuel_calculator.calculate_max_fuel(1_000_000_000_000);

        assert_eq!(expected_fuel, max_fuel);
    }

    fn run_test(chemical_reactions: &Vec<ChemicalReaction>, expected_ore_constituent: usize) {
        let reactor = Reactor::build_reactor(&chemical_reactions);
        let reactor_result = reactor.calculate_reaction(QuantifiedChemical::new(1, "FUEL".into()));
        assert_eq!(
            reactor_result
                .constituent("ORE".into())
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
