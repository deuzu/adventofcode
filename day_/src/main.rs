use std::fs;

fn main() {
    let file_content =fs::read_to_string("exercise.txt").unwrap();
    println!("{}", file_content);
}
