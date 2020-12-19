use std::{collections::HashMap, fs::File, str::Lines};
use std::io::{Error, Read};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Instruction {
    pub kind: String,
    pub value: i32,
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        let mut iter = instruction.split(' ');
        let (kind, value) = (iter.next().unwrap().to_string(), iter.next().unwrap().parse::<i32>().unwrap());

        Instruction {
            kind,
            value,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_instructions(input.lines());
    let mut accumulator = 0;
    let mut cache: HashSet<usize> = HashSet::new();

    execute_instruction(instructions, 0, &mut accumulator, &mut cache);

    accumulator as usize
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_instructions(input.lines());
    let mut cache: HashSet<usize> = HashSet::new();

    let (_, accumulator) = execute_instruction_part2(&instructions, 0, 0, &mut cache, false);

    accumulator as usize
}

pub fn get_file(filepath: &str) -> Result<String, Error> {
    let path = Path::new(filepath);
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

pub fn parse_instructions(lines: Lines) -> HashMap<usize, Instruction> {
    lines.enumerate()
        .fold(HashMap::new(), |mut acc, (i, ins)| {
            acc.insert(i, Instruction::new(ins));

            acc
        })
}

pub fn execute_instruction(instructions: HashMap<usize, Instruction>, index: usize, accumulator: &mut i32, cache: &mut HashSet<usize>) {
    if cache.contains(&index) {
        return;
    };

    cache.insert(index);

    let instruction = match instructions.get(&index) {
        Some(i) => i,
        None => return,
    };

    let mut next_index: i32 = index as i32;

    match &instruction.kind[..] {
        "acc" => {
            *accumulator += instruction.value;

            next_index += 1;
        },
        "jmp" => { next_index += instruction.value; },
        "nop" => { next_index += 1; },
        _ => {},
    };

    execute_instruction(instructions, next_index as usize, accumulator, cache);
}

pub fn execute_instruction_part2(instructions: &HashMap<usize, Instruction>, index: usize, accumulator: i32, cache: &mut HashSet<usize>, alternating: bool) -> (bool, i32) {
    if cache.contains(&index) {
        return (false, accumulator);
    };

    cache.insert(index);

    let instruction = match instructions.get(&index) {
        Some(i) => i,
        None => return (true, accumulator),
    };

    let mut next_index: i32 = index as i32;
    let mut new_accumulator = accumulator.clone();
    let mut alternate_cache = cache.clone();

    match &instruction.kind[..] {
        "acc" => {
            new_accumulator += instruction.value;

            next_index += 1;
        },
        "jmp" => {
            if !alternating {
                let (result, alternate_accumulator) = execute_instruction_part2(instructions, (next_index + 1) as usize, new_accumulator, &mut alternate_cache, true);
                if result {
                    return (result, alternate_accumulator);
                }
            }

            next_index += instruction.value;
        },
        "nop" => {
            if !alternating {
                let (result, alternate_accumulator) = execute_instruction_part2(instructions, (next_index + instruction.value) as usize, new_accumulator, &mut alternate_cache, true);
                if result {
                    return (result, alternate_accumulator);
                }
            }

            next_index += 1;
        },
        _ => {},
    };

    let (result, accumulator) = execute_instruction_part2(instructions, next_index as usize, new_accumulator, cache, alternating);

    (result, accumulator)
}

#[cfg(test)]
mod tests {
    use super::{get_file, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_file("input.txt").unwrap();
        let expected_result = 1262;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_file("input.txt").unwrap();
        let expected_result = 1643;

        assert_eq!(expected_result, part2(&input));
    }
}
