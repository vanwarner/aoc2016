use array2d::*;
/// Day 6 of AoC 2016.
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

fn get_common<'a>(index: usize) -> Vec<char> {
    let mut result: Vec<_> = vec![];
    if let Ok(lines) = read_lines("inputs/day_six") {
        let mut vertical_lines: Vec<Vec<char>> = vec![vec![]];
        for line in lines {
            if let Ok(entry) = line {
                let mut chars: Vec<char> = vec![];
                for char in entry.chars() {
                    chars.push(char);
                }
                vertical_lines.push(chars);
            }
        }
        vertical_lines.remove(0);
        let two_dimensions = Array2D::from_columns(&vertical_lines);
        let input_fixed = two_dimensions.as_rows();
        for row_iter in input_fixed.iter() {
            let mut frequency: std::collections::HashMap<char, u32> =
                std::collections::HashMap::new();
            for element in row_iter {
                *frequency.entry(*element).or_insert(0) += 1;
            }
            let mut most_common: Vec<_> = frequency.iter().collect();
            most_common.sort_by(|a, b| b.1.cmp(a.1));
            //println!("{:?}", most_common);
            result.push(*most_common[index].0);
        }
    }
    result
}

pub fn part_one() {
    let first_letter_most_common = get_common(0 as usize);
    println!("{:?}", first_letter_most_common);
}
pub fn part_two() {
    let last_letter_most_common = get_common(25 as usize);
    println!("{:?}", last_letter_most_common);
}
