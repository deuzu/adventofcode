use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = get_input().expect("Failed to open input file");
    let lines = input.lines();
    let mut tree_count = 0;
    let mut line_position = 0;

    for line in lines {
        let mut land: Vec<char> = line.chars().collect();
        let mut new_line: String = line.to_string().clone();

        while line_position > &land.len() -1 {
            new_line = format!("{}{}", new_line, new_line);
            land = new_line.chars().collect();
        }

        if '#' == land[line_position] {
            tree_count = tree_count + 1;
        }

        line_position = line_position + 3;
    }

    println!("There is {} tree(s) in the path", tree_count);
    assert_eq!(223, tree_count);
}

fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}
