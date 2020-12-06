use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str::Lines;

fn main() {
    let input = get_input().expect("Failed to open input file");
    let lines = input.lines();
    let mut result: u64 = 1;
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    for slope in slopes.iter() {
        let tree_count = count_tree_in_slope(&slope, &lines);
        println!("There is {} tree(s) in the slope {:?}", tree_count, &slope);
        let pre_result = result.clone();
        result = result * u64::from(tree_count);
        println!("The result of {} x {} is {}", pre_result, tree_count, result);
    }

    println!("The result of the multiplication of tree is: {}", result);
    assert_eq!(3517401300, result);
}

fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}

fn count_tree_in_slope(slope: &(usize, usize), lines: &Lines) -> u32 {
    let mut tree_count = 0;
    let mut line_position = 0;
    let new_lines = lines.clone();

    for (i, line) in new_lines.enumerate() {
        if 0 != i % &slope.1 {
            continue;
        }

        let mut land: Vec<char> = line.chars().collect();
        let mut new_line: String = line.to_string().clone();

        while line_position > &land.len() -1 {
            new_line = format!("{}{}", new_line, new_line);
            land = new_line.chars().collect();
        }

        if '#' == land[line_position] {
            tree_count = tree_count + 1;
        }

        line_position = line_position + &slope.0;
    }

    tree_count
}
