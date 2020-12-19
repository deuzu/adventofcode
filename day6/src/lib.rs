use std::{collections::{HashMap, HashSet}, fs::File};
use std::io::{Error, Read};
use std::path::Path;

#[derive(Debug)]
struct UniqueIterator<I> {
    iter: I,
    cache: HashSet<char>,
}

impl<I> Iterator for UniqueIterator<I>
    where I: Iterator<Item=char>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let cache = &mut self.cache;

        match self.iter.find(|i| { !cache.contains(i) }) {
            None => None,
            Some(i) => {
                cache.insert(i);

                Some(i)
            }
        }
    }
}

pub fn get_input() -> Result<String, Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

pub fn part1(input: &str) -> usize {
    let groups = input.split("\n\n");
    let mut sum = 0;

    for group in groups {
        let chars = group.chars().filter(|c| c != &'\n');
        let unique = UniqueIterator {
            iter: chars,
            cache: HashSet::new(),
        };

        sum += unique.count();
    }

    sum
}

pub fn part2(input: &str) -> usize {
    input.split("\n\n").fold(0, |acc, g| {
        let chars = g.chars().filter(|c| c != &'\n');
        let mut chars_count: HashMap<char, usize> = HashMap::new();

        for char in chars {
            let char_sum: &usize = match chars_count.get(&char) {
                Some(c) => c,
                _ => &0,
            };

            let char_sum = *char_sum;

            chars_count.insert(char, char_sum + 1);
        }

        let custom = g.lines().count();
        let filterd_char_count = chars_count.into_iter().filter(|(_, count)| { count == &custom });

        acc + filterd_char_count.count()
    })
}

#[cfg(test)]
mod tests {
    use super::{get_input, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_input().unwrap();
        let expected_result = 6506;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input().unwrap();
        let expected_result = 3243;

        assert_eq!(expected_result, part2(&input));
    }
}
