#[macro_use]
extern crate lazy_static;

use std::{fs::File, collections::{HashMap, HashSet}};
use std::io::{Error, Read};
use std::path::Path;
use regex::Regex;

type Rule = (String, Vec<(usize, String)>);

lazy_static! {
    static ref RE_BAG_CONTAINER: Regex = Regex::new(r"([\w ]+) bag").unwrap();
}

lazy_static! {
    static ref RE_BAG_CONTAINEDS: Regex = Regex::new(r"[\d] ([\w]+ [\w]+) bag(?:s|)").unwrap();
}

lazy_static! {
    static ref RE_BAG_CONTAINEDS_WITH_COUNT: Regex = Regex::new(r"([\d]) ([\w]+ [\w]+) bag(?:s|)").unwrap();
}

pub fn get_input() -> Result<String, Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

pub fn part1(input: &str) -> usize {
    let rules: HashMap<&str, Vec<&str>> = input.lines().fold(HashMap::new(), |mut acc, r| {
        let (container, containeds) = match parse_bags(r) {
            Some(b) => b,
            _ => return acc,
        };

        for contained in containeds {
            match acc.get(contained) {
                Some(c) => {
                    let mut new_vec = vec![container];
                    new_vec.extend(c);

                    acc.insert(contained, new_vec);
                },
                _ => {
                    acc.insert(contained, vec![container]);
                },
            }
        }

        acc
    });

    let cache: HashSet<&str> = HashSet::new();
    let hashset = count_containers(&rules, "shiny gold", &cache);

    hashset.into_iter().count()
}

pub fn part2(input: &str) -> usize {
    let rules: HashMap<String, Vec<(usize, String)>> = input.lines().fold(HashMap::new(), |mut acc, r| {
        match parse_bags_with_count(r) {
            Some((container, containeds)) => {
                acc.insert(container, containeds)
            },
            _ => return acc,
        };

        acc
    });

    count_containeds(&rules, "shiny gold")
}

fn parse_bags(bags: &str) -> Option<(&str, Vec<&str>)> {
    let mut iter = bags.split("contain");
    let (container, containeds): (&str, &str) = (iter.next().unwrap(), iter.next().unwrap());
    let container = RE_BAG_CONTAINER.captures(container).unwrap().get(1).unwrap().as_str();

    let mut containeds_vec: Vec<&str> = Vec::new();
    for contained in containeds.split(",") {
        let caps_containeds = RE_BAG_CONTAINEDS.captures(contained);

        match caps_containeds {
            Some(caps) => {
                containeds_vec.push(caps.get(1).unwrap().as_str());
            },
            _ => (),
        };

    }

    Some((container, containeds_vec))
}

fn parse_bags_with_count(bags: &str) -> Option<Rule> {
    let mut iter = bags.split("contain");
    let (container, containeds): (&str, &str) = (iter.next().unwrap(), iter.next().unwrap());
    let container = RE_BAG_CONTAINER.captures(container).unwrap().get(1).unwrap().as_str();

    let mut containeds_vec = Vec::new();
    for contained in containeds.split(",") {
        let caps_containeds = RE_BAG_CONTAINEDS_WITH_COUNT.captures(contained);

        match caps_containeds {
            Some(caps) => {
                containeds_vec.push((caps.get(1).unwrap().as_str().parse::<usize>().unwrap(), caps.get(2).unwrap().as_str().to_string()));
            },
            _ => (),
        };
    }

    Some((container.to_string(), containeds_vec))
}

fn count_containers<'a>(rules: &HashMap<&'a str, Vec<&'a str>>, contained: &str, cache: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut new_cache: HashSet<&'a str> = HashSet::new();
    new_cache.extend(cache);

    let containers = match rules.get(contained) {
        Some(c) => c.into_iter(),
        _ => return new_cache,
    };

    for container in containers {
        new_cache.insert(container);
        let new_new_cache = count_containers(rules, container, &new_cache);
        new_cache.extend(new_new_cache);
    }

    new_cache
}

fn count_containeds(rules: &HashMap<String, Vec<(usize, String)>>, contained: &str) -> usize {
    let mut count = 0;

    match rules.get(contained) {
        Some(c) => {
            count += c.into_iter().fold(0, |mut acc, c| {
                acc += c.0 + (c.0 * count_containeds(rules, &c.1[..]));

                acc
            });

        },
        None => {},
    };

    count
}

#[cfg(test)]
mod tests {
    use super::{get_input, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_input().unwrap();
        let expected_result = 300;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input().unwrap();
        let expected_result = 1;

        assert_eq!(expected_result, part2(&input));
    }
}
