use std::{fs::File, collections::HashMap};
use std::io::{Error, Read};
use std::path::Path;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seat {
    id: usize,
    code: String,
    row: usize,
    column: usize,
}

impl Seat {
    fn new(seat_code: &str) -> Seat {
        let row_bin = &seat_code[..7].replace("F", "0").replace("B", "1");
        let row_val = usize::from_str_radix(row_bin, 2).unwrap();

        let column_bin = &seat_code[7..].replace("L", "0").replace("R", "1");
        let column_val = usize::from_str_radix(column_bin, 2).unwrap();

        let seat_id = row_val * 8 + column_val;

        Seat {
            id: seat_id,
            code: seat_code.to_string(),
            row: row_val,
            column: column_val,
        }
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.row, &self.column).cmp(&(other.row, &other.column))
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    let seats: Vec<&str> = input.lines().collect();

    let highest_seat_id = seats.into_iter().fold(0, |acc, s| {
        let row_bin = &s[..7].replace("F", "0").replace("B", "1");
        let row_val = usize::from_str_radix(row_bin, 2).unwrap();

        let column_bin = &s[7..].replace("L", "0").replace("R", "1");
        let column_val = usize::from_str_radix(column_bin, 2).unwrap();

        let seat_id = row_val * 8 + column_val;

        if seat_id > acc {
            return seat_id
        }

        acc
    });

    highest_seat_id
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines();
    let mut seats: HashMap<usize, Vec<Seat>> = lines.into_iter().fold(HashMap::new(), |mut acc, s| {
        let seat = Seat::new(s);
        let mut new_seat_row = Vec::new();
        new_seat_row.push(seat.clone());

        match acc.get(&seat.row) {
            Some(s) => new_seat_row.extend(s.clone()),
            None => (),
        };

        acc.insert(seat.row, new_seat_row.clone());

        acc
    });

    seats.remove(&0);
    seats.remove(&seats.len());
    let mut missing_seat_id = None;

    for (row, mut seat_group) in seats {
        if seat_group.clone().into_iter().count() < 8 {
            seat_group.sort();

            let mut old_seat: Option<Seat> = None;

            for seat in &seat_group {
                match old_seat {
                    Some(s) => {
                        if s.column != &seat.column - 1 {
                            missing_seat_id = Some((&row * 8) + (&seat.column - 1));
                        }
                     },
                    _ => ()
                }

                old_seat = Some(seat.clone());
            }
        }
    }

    println!("The ID of your seat is {}", missing_seat_id.unwrap());

    missing_seat_id.unwrap()
}

#[cfg(test)]
mod tests {
    use super::{get_input, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_input().unwrap();
        let expected_result = 813;

        assert_eq!(expected_result, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input().unwrap();
        let expected_result = 612;

        assert_eq!(expected_result, part2(&input));
    }
}
