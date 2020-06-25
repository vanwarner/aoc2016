/// Day 2 of AoC 2016.
/// Calculated 1848 for Part 2, answer was 1849. 
/// TODO: Find that edge case. Maybe. I'm pretty lazy...

// namespacing
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use array2d::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one() {
    let mut num_possible = 0;

    if let Ok(lines) = read_lines("./inputs/day_three") {
        for line in lines {
            if let Ok(entry) = line {
                let mut sides: Vec<i32> = entry
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                sides.sort_unstable();
                if sides[0] + sides[1] > sides[2] {
                    num_possible += 1;
                }
            }
        }
    }
    println!("Num of possible triangles: {}", num_possible);
}

pub fn part_two() {
    let mut num_possible = 0;

    if let Ok(lines) = read_lines("./inputs/day_three") {
        let mut vertical_lines: Vec<Vec<i32>> = vec![vec![]];
        for line in lines {
            if let Ok(entry) = line {
                let mut sizes: Vec<i32> = entry
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                vertical_lines.push(sizes);
            }
            //if sides[0] + sides[1] > sides[2] {
            //    num_possible += 1;
            //}
        }
        vertical_lines.remove(0);
        let mut triangles = Array2D::from_columns(&vertical_lines);
        let sides = triangles.as_rows();
        //println!("{:?}", sides); 
        for mut row in sides.into_iter(){
            while row.len() > 0 {
                let mut three = row.split_off(row.len() - 3);
                three.sort_unstable();
                if three[0] + three[1] > three[2] {
                    num_possible += 1;
                }
                println!("{:?}\n", three);
            }
        }
        
    }
    println!("Num of possible triangles: {}", num_possible);
}
