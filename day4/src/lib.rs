#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use regex::{Captures, Regex, RegexSet, SetMatches};

lazy_static! {
    static ref RE_SET_FIELDS: RegexSet = RegexSet::new(&[
        r"byr:",
        r"iyr:",
        r"eyr:",
        r"hgt:",
        r"hcl:",
        r"ecl:",
        r"pid:",
    ]).unwrap();
}

lazy_static! {
    static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
}

lazy_static! {
    static ref RE_HCL: Regex = Regex::new(r"^\#[a-f0-9]{6}$").unwrap();
}

lazy_static! {
    static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
}

lazy_static! {
    static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

pub fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

pub fn part1(input: &String) -> u32 {
    let passports = input.split("\n\n");
    let mut valid_passports = 0;

    for passport in passports {
        let normalized_passport = passport
            .split("\n")
            .fold(String::new(), |acc, fields| format!("{} {}", acc, fields));
        if normalized_passport.contains("byr:")
            && normalized_passport.contains("iyr:")
            && normalized_passport.contains("eyr:")
            && normalized_passport.contains("hgt:")
            && normalized_passport.contains("hcl:")
            && normalized_passport.contains("ecl:")
            && normalized_passport.contains("pid:") {
            valid_passports = valid_passports + 1;
        }
    }

    valid_passports
}

pub fn part2(input: &String) -> u32 {
    let mut valid_passports = 0;
    let passports = input.split("\n\n");

    'passport: for passport in passports {
        let normalized_passport = passport
            .split("\n")
            .fold(String::new(), |acc, fields| format!("{} {}", acc, fields).trim().to_string());

        if !has_mandatory_fields(passport) {
            continue;
        }

        let fields = normalized_passport.split(" ");

        for field in fields {
            if !field_is_valid(field) {
                continue 'passport;
            }
        }

        valid_passports += 1;
    }

    valid_passports
}

fn has_mandatory_fields(passport: &str) -> bool {
    let caps: SetMatches = RE_SET_FIELDS.matches(passport);

    return 7 == caps.into_iter().count()
}

fn field_is_valid(field: &str) -> bool {
    let mut key_value = field.split(":");
    let key = key_value.next();
    let value = match key_value.next() {
        Some(v) => v,
        _ => return false,
    };

    match key {
        Some("byr") => return is_valid_number_in_range(value, 1920, 2002),
        Some("iyr") => return is_valid_number_in_range(value, 2010, 2020),
        Some("eyr") => return is_valid_number_in_range(value, 2020, 2030),
        Some("hgt") => return is_valid_height(value),
        Some("hcl") => return is_valid_hair_color(value),
        Some("ecl") => return is_valid_eye_color(value),
        Some("pid") => return is_valid_passport_id(value),
        Some(_) => return true,
        None => return false,
    }
}

fn is_valid_number_in_range(number: &str, min: u16, max: u16) -> bool {
    let number_int = number.parse::<u16>();

    if number_int.is_err() {
        return false;
    }

    let number_value = number_int.unwrap();

    if number_value < min || number_value > max {
        return false;
    }

    true
}

fn is_valid_height(height: &str) -> bool {
    let caps: Option<Captures> = RE_HGT.captures(height);

    match caps {
        Some(c) => {
            let height: u8 = match c.get(1).unwrap().as_str().parse() {
                Ok(h) => h,
                _ => return false,
            };

            let unit = c.get(2).unwrap().as_str();

            match unit {
                "cm" => return height >= 150 && height <= 193,
                "in" => return height >= 59 && height <= 76,
                _ => (),
            }
        },
        _ => (),
    }

    false
}

fn is_valid_hair_color(hair_color: &str) -> bool {
    RE_HCL.captures(hair_color).is_some()
}

fn is_valid_eye_color(eye_color: &str) -> bool {
    RE_ECL.captures(eye_color).is_some()
}

fn is_valid_passport_id(passport_id: &str) -> bool {
    RE_PID.captures(passport_id).is_some()
}

#[cfg(test)]
mod tests {
    use super::{get_input, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_input().unwrap();
        let expected_result = 228;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input().unwrap();
        let expected_result = 175;

        assert_eq!(expected_result, part2(&input));
    }
}
