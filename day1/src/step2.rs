use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = get_input().expect("Failed to open input file");
    let lines = input.lines().into_iter();
    let mut result: Option<u32> = None;

    for t1 in lines.clone() {
        for t2 in lines.clone() {
            for t3 in lines.clone() {
                let pair: (u32, u32, u32) = (t1.parse().unwrap(), t2.parse().unwrap(), t3.parse().unwrap());

                if 2020 == pair.0 + pair.1 + pair.2 {
                    println!("A pair that is equal to 2020 has been found: {:?}", pair);
                    println!("Multiplying together the numbers in the pair results in: {}", pair.0 * pair.1 * pair.2);
                    result = Some(pair.0 * pair.1 * pair.2);
                }
            }
        }
    }

    assert_eq!(Some(44211152), result);
}

fn get_input() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    Ok(input)
}
