/// Day 4 of AoC 2016.
// namespacing
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Room<'a> {
    encrypted_name: Vec<&'a str>,
    sector_id: i32,
    checksum: &'a str,
}

fn parse_to_room(input: &str) -> Result<Room, io::Error> {
    let mut room_code: Vec<&str> = input.split('-').collect();
    let last: Vec<&str> = room_code
        .pop()
        .expect("Couldn't pop last element")
        .split('[')
        .collect();
    let sector = last[0].parse::<i32>().expect("Couln't parse to i32");
    let checksum = last[1].trim_end_matches("]");
    Ok(Room {
        encrypted_name: room_code,
        sector_id: sector,
        checksum: checksum,
    })
}

fn calculate_checksum(room: &Room) -> String {
    let mut frequency: std::collections::HashMap<char, u32> = std::collections::HashMap::new();
    for element in &room.encrypted_name {
        for char in element.chars() {
            *frequency.entry(char).or_insert(0) += 1;
        }
    }
    let mut count_vec: Vec<_> = frequency.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1)); // sort by max frequency
    let mut sum: Vec<_> = vec![];
    while sum.len() < 5 {
        if count_vec.len() == 1 {
            sum.push(count_vec[0].0);
            break;
        }
        if count_vec[0].1 > count_vec[1].1 {
            // if frequency is explicitly larger than next value
            sum.push(count_vec[0].0);
            count_vec.remove(0);
        } else {
            if count_vec[0].1 == count_vec[1].1 {
                // if frequency is same as next value
                let count = count_vec[0].1; // store freqeuncy value
                let mut prev_lowest_char = count_vec[0].0; // store character
                let mut index = 0; // store iterator position in tuple vector
                for (i, char) in count_vec.iter().enumerate() {
                    // lazy search through all entries with frequency matching 'count'
                    if char.1 == count && char.0 < prev_lowest_char {
                        prev_lowest_char = char.0;
                        index = i;
                    }
                }
                sum.push(prev_lowest_char);
                count_vec.remove(index);
                // I'm too lazy to install nightly, we're gonna search through the
                // tuple vector and remove matches for this character we found lol
            }
        }
    }
    let stringsum = sum.iter().map(|x| x.to_string()).collect::<String>();
    return stringsum;
}

fn shift_cipher(name: Vec<&str>, amt: i32) -> Vec<String> {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let shift = amt % 26;
    let mut output: Vec<String> = vec![];
    for element in name {
        let mut result = String::new();
        for char in element.chars() {
            match alphabet.chars().position(|b| char == b) {
                Some(x) => {
                    let idx: usize = shift as usize + x;
                    let new_index = if (idx as u32) >= 26u32 {
                        idx - 26usize
                    } else {
                        idx
                    };
                    match alphabet.chars().nth(new_index) {
                        Some(x) => {
                            &mut result.push(x);
                        }
                        None => {
                            panic!("No element found at index {}", new_index);
                        }
                    };
                }
                None => {
                    panic!("'{}' is not a valid element in the alphabet", char);
                }
            };
        }
        output.push(result);
    }
    return output;
}

pub fn part_one() {
    let mut total_sum = 0;
    if let Ok(lines) = read_lines("inputs/day_four") {
        for line in lines {
            if let Ok(entry) = line {
                let room = parse_to_room(&entry).expect("Failed to generate room from input");
                let stringsum = calculate_checksum(&room);
                if stringsum == room.checksum {
                    total_sum += room.sector_id;
                }
            }
        }
    }
    println!("Sum of valid room IDs: {}", total_sum);
}

pub fn part_two() {
    if let Ok(lines) = read_lines("inputs/day_four") {
        for line in lines {
            if let Ok(entry) = line {
                let room = parse_to_room(&entry).expect("Failed to generate room from input");
                let room_name = shift_cipher(room.encrypted_name, room.sector_id);
                if room_name == vec!["northpole", "object", "storage"] {
                    println!("{:?}", room.sector_id);
                }
            }
        }
    }
}
