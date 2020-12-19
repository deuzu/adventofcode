use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

/// This is the part 1 of the exercise
/// # Examples
/// ```rust
/// #[test]
/// fn test_part1() {
///     let input = get_file("input.txt").unwrap();
///     let expected_result = 133015568;
///     assert_eq!(expected_result, part1(&input));
/// }
/// ```
pub fn part1(input: &str) -> usize {
    let mut iter = input.lines().into_iter();
    let mut invalid_number: Option<u64> = None;

    while let Some(_) = iter.next() {
        let term_result = iter.clone().take(26);
        let terms = term_result.clone().take(25);
        let result = term_result.clone().skip(25).take(1).next().unwrap().parse::<u64>().unwrap();
        let mut sum_terms_result = false;

        for i in terms.clone() {
            let i = i.parse::<u64>().unwrap();

            for j in terms.clone() {
                let j = j.parse::<u64>().unwrap();

                if i == j {
                    continue;
                }

                sum_terms_result = sum_terms_result || i + j == result
            }
        }

        if !sum_terms_result {
            invalid_number = Some(result);

            break;
        }
    }

    invalid_number.unwrap() as usize
}

pub fn part2(input: &str) -> usize {
    let mut iter = input.lines().into_iter().map(|n| -> u64 { n.parse().unwrap() });
    let invalid_number = 133015568;
    let mut weakness: Option<u64> = None;

    while let Some(n) = iter.next() {
        if n == invalid_number {
            break;
        }

        let mut weakness_acc = 0;
        let mut i = 2;

        while weakness_acc < invalid_number {
            let contiguous_iter = iter.clone().take(i);
            weakness_acc = contiguous_iter.clone().fold(0, |mut acc, n| {
                acc += n;

                acc
            });

            if weakness_acc == invalid_number {
                let min = contiguous_iter.clone().min().unwrap();
                let max = contiguous_iter.clone().max().unwrap();
                weakness = Some(min + max);

                break;
            }

            i += 1;
        }
    }

    weakness.unwrap() as usize
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
        let expected_result = 133015568;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_file("input.txt").unwrap();
        let expected_result = 16107959;

        assert_eq!(expected_result, part2(&input));
    }
}
