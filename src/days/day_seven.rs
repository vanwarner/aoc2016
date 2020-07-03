/// Day 7 of AoC 2016.
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

fn get_ipv7_inputs<'a>() -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![vec![]];
    if let Ok(lines) = read_lines("inputs/day_seven") {
        for line in lines {
            if let Ok(entry) = line {
                let split_line: Vec<String> = entry
                    .split(|c| c == ']' || c == '[')
                    .map(|c| c.to_string())
                    .collect();
                result.push(split_line);
            }
        }
    }
    result.remove(0);
    result
}

fn valid_ipv7(ipv7: Vec<String>) -> Vec<bool> {
    let mut flags: Vec<bool> = vec![];
    for slice in 0..ipv7.len() {
        let mut flag: bool;
        let hypernet = &ipv7[slice];
        if slice % 2 == 1 {
            // slice in brackets
            flag = true;
            for char in 0..(hypernet.len() - 3) {
                if hypernet.chars().nth(char) == hypernet.chars().nth(char + 3)
                    && hypernet.chars().nth(char + 1) == hypernet.chars().nth(char + 2)
                    && hypernet.chars().nth(char) != hypernet.chars().nth(char + 1)
                {
                    flag = false;
                    break;
                }
            }
        } else {
            // slice outside brackets
            flag = false;
            for char in 0..(hypernet.len() - 3) {
                if hypernet.chars().nth(char) == hypernet.chars().nth(char + 3)
                    && hypernet.chars().nth(char + 1) == hypernet.chars().nth(char + 2)
                    && hypernet.chars().nth(char) != hypernet.chars().nth(char + 1)
                {
                    flag = true;
                    break;
                }
            }
        }
        flags.push(flag);
    }
    flags
}

fn valid_ssl(ipv7: Vec<String>) -> bool {
    let mut aba: Vec<_> = vec![];
    for slice in (0..ipv7.len()).step_by(2) {
        // look at supernet first
        let supernet = &ipv7[slice];
        for char in 0..(supernet.len() - 2) {
            if supernet.chars().nth(char) == supernet.chars().nth(char + 2)
                && supernet.chars().nth(char) != supernet.chars().nth(char + 1)
            {
                aba.push((supernet.chars().nth(char), supernet.chars().nth(char + 1)));
            }
        }
    }
    if aba.len() == 0 {
        return false;
    } else {
        for slice in (1..ipv7.len()).step_by(2) {
            let hypernet = &ipv7[slice];
            for char in 0..(hypernet.len() - 2) {
                for tuple in aba.iter() {
                    if hypernet.chars().nth(char) == tuple.1
                        && hypernet.chars().nth(char + 1) == tuple.0
                        && hypernet.chars().nth(char + 2) == tuple.1
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn part_one() {
    let test = get_ipv7_inputs();
    let mut sum = 0;
    for line in test.iter() {
        let bools = valid_ipv7(line.to_vec());
        println!("{:?}", bools);
        let mut or: bool = false;
        let mut skip: bool = false;
        for value in 0..bools.len() {
            if value % 2 == 1 && bools[value] == false {
                skip = true;
                break;
            } else if value % 2 == 0 {
                or = or || bools[value];
            }
        }
        if or == true && skip == false {
            sum += 1
        }
    }
    println!("Number of valid IPv7: {}", sum);
}

pub fn part_two() {
    let test = get_ipv7_inputs();
    let mut sum = 0;
    for line in test.iter() {
        let bool = valid_ssl(line.to_vec());
        if bool {
            sum += 1;
        }
    }
    println!("Number of valid SSL: {}", sum);
}
