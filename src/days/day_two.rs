use array2d::*;
/// Day 2 of AoC 2016.
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

struct Pos {
    row: usize,
    col: usize,
}

pub fn part_one() {
    let mut passcode: Vec<usize> = vec![];
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let numpad = Array2D::from_rows(&rows);
    let mut pos = Pos { row: 1, col: 1 }; //start in the middle of numpad

    if let Ok(lines) = read_lines("./inputs/day_two") {
        for line in lines {
            if let Ok(instruction) = line {
                for char in instruction.chars() {
                    match char {
                        'L' => {
                            if pos.col > 0 {
                                pos.col -= 1;
                            }
                        }
                        'R' => {
                            if pos.col < 2 {
                                pos.col += 1;
                            }
                        }
                        'U' => {
                            if pos.row > 0 {
                                pos.row -= 1;
                            }
                        }
                        'D' => {
                            if pos.row < 2 {
                                pos.row += 1;
                            }
                        }
                        _ => println!("EOL/EOF?"), // should never reach
                    }
                }
            }
            passcode.push(numpad[(pos.row, pos.col)]);
        }
    }
    println!("{:?}", passcode);
}

pub fn part_two() {
    let mut passcode: Vec<&str> = vec![];
    let rows = vec![
        vec!["0", "0", "1", "0", "0"],
        vec!["0", "2", "3", "4", "0"],
        vec!["5", "6", "7", "8", "9"],
        vec!["0", "A", "B", "C", "0"],
        vec!["0", "0", "D", "0", "0"],
    ];
    let numpad = Array2D::from_rows(&rows);
    let mut pos = Pos { row: 2, col: 0 }; //start @ 5

    if let Ok(lines) = read_lines("./inputs/day_two") {
        for line in lines {
            if let Ok(instruction) = line {
                for char in instruction.chars() {
                    match char {
                        'L' => {
                            if pos.col > 0 && numpad[(pos.row, pos.col - 1)] != "0" {
                                pos.col -= 1;
                            }
                        }
                        'R' => {
                            if pos.col < 4 && numpad[(pos.row, pos.col + 1)] != "0" {
                                pos.col += 1;
                            }
                        }
                        'U' => {
                            if pos.row > 0 && numpad[(pos.row - 1, pos.col)] != "0" {
                                pos.row -= 1;
                            }
                        }
                        'D' => {
                            if pos.row < 4 && numpad[(pos.row + 1, pos.col)] != "0" {
                                pos.row += 1;
                            }
                        }
                        _ => println!("EOL/EOF?"), // should never reach
                    }
                }
            }
            passcode.push(numpad[(pos.row, pos.col)]);
        }
    }
    println!("{:?}", passcode);
}
