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
    assert_eq!(410, valid_password_count);
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

    let letter_count_range = split.next().unwrap();
    let mut letter_count_range_split = letter_count_range.split("-");
    let letter_count_range_min = letter_count_range_split.next().unwrap().parse().unwrap();
    let letter_count_range_max = letter_count_range_split.next().unwrap().parse().unwrap();
    let mut letter_policy = split.next().unwrap().to_string();
    letter_policy.pop();
    let password = split.next().unwrap();

    let letter_contained_count = password.matches(&letter_policy).count();

    letter_contained_count >= letter_count_range_min && letter_contained_count <= letter_count_range_max
}
