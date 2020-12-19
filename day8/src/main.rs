use std::fs;

fn main() {
    let url = "https://adventofcode.com/2020/day/8";
    let file_content =fs::read_to_string("exercise.txt").unwrap();

    println!("{}", url);
    println!("{}", file_content);
}
