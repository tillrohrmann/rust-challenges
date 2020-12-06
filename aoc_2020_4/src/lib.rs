use std::collections::HashMap;
use aoc_common::GenericResult;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<PassportFields, String>,
}

impl Passport {
    pub fn new(fields: HashMap<PassportFields, String>) -> Passport {
        Passport {
            fields,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.fields.len() == 8 ||
            (self.fields.len() == 7 && !self.fields.contains_key(&PassportFields::CountryId))
    }

    pub fn parse_passport(key_value_pairs: &str) -> GenericResult<Passport> {
        let splits = key_value_pairs.split(" ");
        let mut fields = HashMap::new();

        for split in splits {
            let mut key_value = split.split(":");

            let key = key_value.next().ok_or("Did not find key.")?;
            let value = key_value.next().ok_or("Did not find value.")?;

            fields.insert(key.try_into()?, value.to_string());
        }

        Ok(Passport::new(fields))
    }
}

pub fn parse_passports(path: &str) -> GenericResult<Vec<Passport>> {
    let lines = aoc_common::read_raw_file_content(path)?;
    parse_passports_from_lines(&lines)
}

fn parse_passports_from_lines(lines: &Vec<String>) -> GenericResult<Vec<Passport>> {
    let mut first_line = 0;
    let mut passports = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            let passport = Passport::parse_passport(&lines[first_line..idx].join(" "))?;
            passports.push(passport);
            first_line = idx + 1;
        }
    }

    if first_line < lines.len() {
        let passport = Passport::parse_passport(&lines[first_line..lines.len()].join(" "))?;
        passports.push(passport);
    }

    Ok(passports)
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PassportFields {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl TryFrom<&str> for PassportFields {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "byr" => Ok(PassportFields::BirthYear),
            "iyr" => Ok(PassportFields::IssueYear),
            "eyr" => Ok(PassportFields::ExpirationYear),
            "hgt" => Ok(PassportFields::Height),
            "hcl" => Ok(PassportFields::HairColor),
            "ecl" => Ok(PassportFields::EyeColor),
            "pid" => Ok(PassportFields::PassportId),
            "cid" => Ok(PassportFields::CountryId),
            x => Err(format!("Unknown field {}", x))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_passports_from_lines;

    #[test]
    fn simple_example() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let input: Vec<String> = input.split("\n").map(|i| i.to_string()).collect();

        let passports = parse_passports_from_lines(&input).unwrap();

        assert_eq!(passports.iter().filter(|passport| passport.is_valid()).count(), 2);
    }
}
