use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = get_input().expect("Failed to open input file");
    let lines = input.lines().into_iter();
    let mut result: Option<u32> = None;

    for p1 in lines.clone() {
        for p2 in lines.clone() {
            let pair: (u32, u32) = (p1.parse().unwrap(), p2.parse().unwrap());

            if 2020 == pair.0 + pair.1 {
                println!("A pair that is equal to 2020 has been found: {:?}", pair);
                println!("Multiplying together the numbers in the pair results in: {}", pair.0 * pair.1);
                result = Some(pair.0 * pair.1);
            }
        }
    }

    assert_eq!(Some(1009899), result);
}

fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}
