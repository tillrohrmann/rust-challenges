#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use aoc_common::GenericResult;

use regex::Regex;

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<PassportFields, String>,
}

pub trait Validator {
    fn validate_passport(&self, passport: &Passport) -> GenericResult<bool>;
}

pub struct SimpleValidator {}

impl Validator for SimpleValidator {
    fn validate_passport(&self, passport: &Passport) -> GenericResult<bool> {
        Ok(passport.fields.len() == 8 ||
            (passport.fields.len() == 7 && !passport.fields.contains_key(&PassportFields::CountryId)))
    }
}

impl SimpleValidator {
    pub fn new() -> SimpleValidator {
        SimpleValidator{}
    }
}

pub struct Part2Validator {}

impl Validator for Part2Validator {
    fn validate_passport(&self, passport: &Passport) -> GenericResult<bool> {
        let fields_validations: Vec<(PassportFields, fn(&Part2Validator, &String) -> GenericResult<bool>)> = vec![
            (PassportFields::Height, Part2Validator::validate_height),
            (PassportFields::BirthYear, Part2Validator::validate_birth_year),
            (PassportFields::IssueYear, Part2Validator::validate_issue_year),
            (PassportFields::ExpirationYear, Part2Validator::validate_expiration_year),
            (PassportFields::HairColor, Part2Validator::validate_hair_color),
            (PassportFields::EyeColor, Part2Validator::validate_eye_color),
            (PassportFields::PassportId, Part2Validator::validate_passport_id)];

        let results: GenericResult<Vec<bool>> = fields_validations.iter()
            .map(|(key, validator)| passport.fields
                .get(key)
                .map(|value| {
                    validator(self, value)
                })
                .unwrap_or(Ok(false)))
            .collect();

        results.map(|values| {
            let result = values.iter().all(|x| *x);
            result
        })
    }
}

impl Part2Validator {
    pub fn new() -> Part2Validator {
        Part2Validator {}
    }

    fn validate_height(&self, value: &String) -> GenericResult<bool> {
        if value.ends_with("in") || value.ends_with("cm") {
            let (height_string, unit) = value.split_at(value.len() - 2);

            let height = height_string.parse::<u32>()?;

            let result =
            if unit == "in" {
                Ok(59 <= height && height <= 76)
            } else {
                Ok(150 <= height && height <= 193)
            };

            result
        } else {
            Ok(false)
        }
    }

    fn validate_birth_year(&self, value: &String) -> GenericResult<bool> {
        if value.len() == 4 {
            let year: i32 = value.parse()?;

            let result = Ok(1920 <= year && year <= 2002);

            result
        } else {
            Ok(false)
        }
    }

    fn validate_issue_year(&self, value: &String) -> GenericResult<bool> {
        if value.len() == 4 {
            let year: i32 = value.parse()?;

            Ok(2010 <= year && year <= 2020)
        } else {
            Ok(false)
        }
    }

    fn validate_expiration_year(&self, value: &String) -> GenericResult<bool> {
        if value.len() == 4 {
            let year: i32 = value.parse()?;

            Ok(2020 <= year && year <= 2030)
        } else {
            Ok(false)
        }
    }

    fn validate_hair_color(&self, value: &String) -> GenericResult<bool> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        let is_match = RE.is_match(value);

        Ok(is_match)
    }

    fn validate_eye_color(&self, value: &String) -> GenericResult<bool> {
        match value.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                Ok(true)
            },
            _ => Ok(false)
        }
    }

    fn validate_passport_id(&self, value: &String) -> GenericResult<bool> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        let is_match = RE.is_match(value);

        Ok(is_match)
    }
}

impl Passport {
    pub fn new(fields: HashMap<PassportFields, String>) -> Passport {
        Passport {
            fields,
        }
    }

    pub fn parse_passport(key_value_pairs: &str) -> GenericResult<Passport> {
        let splits = key_value_pairs.split(" ");
        let mut fields = HashMap::new();

        for split in splits {
            let mut key_value = split.split(":");

            let key = key_value.next().ok_or("Did not find key.")?.trim();
            let value = key_value.next().ok_or("Did not find value.")?.trim();

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
    use super::*;

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

        let validator = SimpleValidator::new();

        assert_eq!(passports.iter().filter(|passport| validator.validate_passport(passport).unwrap()).count(), 2);
    }

    #[test]
    fn valid_examples() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let input: Vec<String> = input.split("\n").map(|i| i.to_string()).collect();

        let passports = parse_passports_from_lines(&input).unwrap();

        let validator = Part2Validator::new();

        assert_eq!(passports.iter().filter(|passport| validator.validate_passport(passport).unwrap()).count(), 4);
    }

    #[test]
    fn invalid_examples() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let input: Vec<String> = input.split("\n").map(|i| i.to_string()).collect();

        let passports = parse_passports_from_lines(&input).unwrap();

        let validator = Part2Validator::new();

        assert_eq!(passports.iter().filter(|passport| validator.validate_passport(passport).unwrap()).count(), 0);
    }
}
