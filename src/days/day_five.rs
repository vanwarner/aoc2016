use md5::{Digest, Md5};
///Day 5 of Advent of Code 2016
use std::char;

pub fn part_one() {
    let mut door_code: String = String::new();
    let mut idx = 0;
    while door_code.len() < 8 {
        let mut door_id: String = String::from("wtnhxymk");
        let mut hasher = Md5::new();
        door_id.push_str(&(idx.to_string()));
        hasher.update(&door_id);
        let result = hasher.finalize();
        if result[0] == 0x00u8 && result[1] == 0x00u8 && result[2] <= 0x0fu8 {
            println!("result = {:#x?}, idx = {}", result, idx);
            door_code.push(char::from_digit(result[2] as u32, 16).unwrap());
        }
        idx += 1;
    }
    println!("door code: {}", door_code);
}
pub fn part_two() {
    let mut door_code: Vec<char> = vec!['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'];
    let mut idx = 0;
    let mut count = 0;
    while count < 8 {
        let mut door_id: String = String::from("wtnhxymk");
        let mut hasher = Md5::new();
        door_id.push_str(&(idx.to_string()));
        hasher.update(&door_id);
        let result = hasher.finalize();
        if result[0] == 0x00u8 && result[1] == 0x00u8 && result[2] <= 0x07u8 {
            if door_code[result[2] as usize] == 'x' {
                // only use first result
                println!("result = {:#x?}, idx = {}", result, idx);
                door_code[result[2] as usize] =
                    char::from_digit((result[3] >> 4) as u32, 16).unwrap();
                count += 1;
            }
        }
        idx += 1;
    }
    println!("door code: {:?}", door_code);
}
