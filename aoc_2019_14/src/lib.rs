use std::convert::TryFrom;
use std::io::ErrorKind;
use std::str::FromStr;
use std::{fmt, io};
use std::num::ParseIntError;

type Chemical = String;

#[derive(Debug)]
struct QuantifiedChemical {
    amount: usize,
    chemical: Chemical,
}

impl QuantifiedChemical {
    fn new(chemical: Chemical, amount: usize) -> QuantifiedChemical {
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

        Ok(QuantifiedChemical::new(chemical, amount))
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
    aoc_common::read_raw_file_content(path)?
        .into_iter()
        .map(|line| line.parse().map_err(|err| io::Error::from(err)))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
