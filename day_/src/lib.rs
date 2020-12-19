use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;


pub fn part1(_input: &str) -> usize {
    1
}

pub fn part2(_input: &str) -> usize {
    1
}

pub fn get_file(filepath: &str) -> Result<String, Error> {
    let path = Path::new(filepath);
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::{get_file, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_file("input.txt").unwrap();
        let expected_result = 1;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_file("input.txt").unwrap();
        let expected_result = 1;

        assert_eq!(expected_result, part2(&input));
    }
}
