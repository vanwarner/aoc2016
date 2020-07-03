use array2d::*;
/// Day 6 of AoC 2016.
/// Calculated 1848 for Part 2, answer was 1849.
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

pub fn part_one() {
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
        println!("{:?}", vertical_lines);
        let two_dimensions = Array2D::from_columns(&vertical_lines);
        let input_fixed = two_dimensions.as_rows();
        println!("{:?}", input_fixed);
        for row_iter in input_fixed.iter() {
            let mut frequency: std::collections::HashMap<char, u32> =
                std::collections::HashMap::new();
            for element in row_iter {
                *frequency.entry(*element).or_insert(0) += 1;
            }
            let mut most_common: Vec<_> = frequency.iter().collect();
            most_common.sort_by(|a, b| b.1.cmp(a.1));
            println!("{:?}", most_common);
        }
    }
}
pub fn part_two() {
    // asks for the least common letter, since I'm printing the entire frequency map just look in
    // the last column lol
    part_one();
}
