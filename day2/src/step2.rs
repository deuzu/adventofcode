use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = get_input().expect("Failed to open input file");
    let lines = input.lines();
    let mut valid_password_count = 0;

    for line in lines {
        if is_password_valid(line) {
            valid_password_count = valid_password_count + 1;
        }
    }

    println!("{} password(s) are valid", valid_password_count);
    assert_eq!(694, valid_password_count);
}

fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

fn is_password_valid(line: &str) -> bool {
    let mut split = line.split(" ");

    let letter_positions = split.next().unwrap();
    let mut letter_positions_split = letter_positions.split("-");
    let letter_position_first: usize = letter_positions_split.next().unwrap().parse().unwrap();
    let letter_position_second: usize = letter_positions_split.next().unwrap().parse().unwrap();
    let mut letter_policy = split.next().unwrap().to_string();
    letter_policy.pop();
    let password = split.next().unwrap();

    let letter_collection: Vec<char> = password.chars().collect();
    let mut letters_position_match = false;

    if let Some(letter) = letter_collection.get(letter_position_first - 1) {
        letters_position_match = letter.to_string() == letter_policy;
    }

    if let Some(letter) = letter_collection.get(letter_position_second - 1) {
        letters_position_match = letters_position_match ^ (letter.to_string() == letter_policy);
    }

    letters_position_match
}
