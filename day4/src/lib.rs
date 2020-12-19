#[macro_use]
extern crate lazy_static;

use regex::{Captures, Regex};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

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

pub fn part1(input: &String) -> usize {
    let passports = input.split("\n\n");
    let valid_passports = passports.filter(|p| is_passport_valid_part1(*p));

    valid_passports.count()
}

pub fn part2(input: &str) -> usize {
    let passports = input.split("\n\n");
    let valid_passports = passports.filter(|p| is_passport_valid_part2(*p));

    valid_passports.count()
}

pub fn part2_multi_thread(input: &String, num_threads: usize) -> usize {
    let passports = input.split("\n\n");
    let passports: Vec<String> = passports.into_iter().map(|x| x.to_string()).collect();
    let valid_passports = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for chunk in passports.chunks(passports.len() / num_threads) {
        let valid = valid_passports.clone();
        let vec = Vec::from(chunk);
        let handle = thread::spawn(move || {
            let valid_passport_chunk = vec.iter().filter(|p| is_passport_valid_part2(*p)).count();
            if let Ok(mut valid) = valid.lock() {
                *valid += valid_passport_chunk;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let valid_passports = *valid_passports.lock().unwrap();

    valid_passports
}

fn is_passport_valid_part1(passport: &str) -> bool {
    let fields: Vec<&str> = passport.split(|c| c == '\n' || c == ' ').collect();

    has_mandatory_fields(&fields)
}

fn is_passport_valid_part2(passport: &str) -> bool {
    let fields: Vec<&str> = passport.split(|c| c == '\n' || c == ' ').collect();

    has_mandatory_fields(&fields) && fields_are_valid(&fields)
}

fn has_mandatory_fields(fields: &Vec<&str>) -> bool {
    let required_fields = fields.iter().filter(|f| !f.contains("cid"));

    7 <= required_fields.count()
}

fn fields_are_valid(fields: &Vec<&str>) -> bool {
    !fields.iter().any(|f| !field_is_valid(f))
}

fn field_is_valid(field: &str) -> bool {
    let mut iter = field.split(":");
    let (key, value) = (iter.next(), iter.next());

    match (key, value) {
        (Some(key), Some(value)) => {
            match key {
                "byr" => is_valid_number_in_range(value, 1920, 2002),
                "iyr" => is_valid_number_in_range(value, 2010, 2020),
                "eyr" => is_valid_number_in_range(value, 2020, 2030),
                "hgt" => is_valid_height(value),
                "hcl" => is_valid_hair_color(value),
                "ecl" => is_valid_eye_color(value),
                "pid" => is_valid_passport_id(value),
                _ => true,
            }
        },
        _ => true,
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
    use super::{get_input, part1, part2, part2_multi_thread};

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

    #[test]
    fn test_part2_multi_thread() {
        let input = get_input().unwrap();
        let expected_result = 175;

        assert_eq!(expected_result, part2_multi_thread(&input, 4));
    }
}
