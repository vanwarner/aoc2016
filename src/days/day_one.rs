/// Day 1 of AoC 2016.
#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    dir: Direction,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}), facing {:?}", self.x, self.y, self.dir)
    }
}

impl Position {
    /// create a new position
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            dir: Direction::North,
        }
    }

    /// update our direction and position
    fn update(&mut self, direction: &char, steps: isize) {
        // update the direction we're facing
        match direction {
            'L' => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::West => self.dir = Direction::South,
                Direction::South => self.dir = Direction::East,
                Direction::East => self.dir = Direction::North,
                _ => eprintln!("Broke out of enum, nice work idiot"),
            },
            'R' => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::North,
                _ => eprintln!("Broke out of enum, nice work idiot"),
            },
            _ => eprintln!("Not a valid direction!"),
        }
        // update the position (x, y) with the rest of the input
        match self.dir {
            Direction::North => self.y += steps,
            Direction::South => self.y -= steps,
            Direction::East => self.x += steps,
            Direction::West => self.x -= steps,
            _ => eprintln!("Stop leaving my enum moron"),
        }
    }
}

pub fn part_one() {
    let mut mypos = Position::new();
    let file = std::fs::read_to_string("./inputs/day_one").unwrap();
    let inputs: Vec<&str> = file.split(',').collect();
    for idx in 0..inputs.len() {
        //        println!("{}", inputs[idx]);
        mypos.update(
            &inputs[idx].trim().chars().nth(0).unwrap(),
            inputs[idx]
                .trim()
                .get(1..)
                .unwrap()
                .parse::<isize>()
                .expect("Couldn't convert input string to isize"),
        );
    }
    println!(
        "The Manhattan Distance from (0, 0) is: {}",
        mypos.x.abs() + mypos.y.abs()
    );
}

pub fn part_two() {
    let mut mypos = Position::new();
    let mut oldpos = Position::new();
    // using a hashmap instead of something smart because I am not smart
    let mut visited = std::collections::HashMap::new();
    let file = std::fs::read_to_string("./inputs/day_one").unwrap();
    let inputs: Vec<&str> = file.split(',').collect();
    //    let inputs: Vec<&str> = vec!["R8", "R4", "R4", "R8"];
    'outer: for idx in 0..inputs.len() {
        //        println!("{}", inputs[idx]);
        mypos.update(
            &inputs[idx].trim().chars().nth(0).unwrap(),
            inputs[idx]
                .trim()
                .get(1..)
                .unwrap()
                .parse::<isize>()
                .expect("Couldn't convert input string to isize"),
        );
        match (mypos.x - oldpos.x, mypos.y - oldpos.y) {
            (0, _) => {
                if oldpos.y < mypos.y {
                    for y in oldpos.y + 1..mypos.y {
                        match visited.contains_key(&(mypos.x, y)) {
                            true => {
                                println!(
                                    "HQ at ({}, {}), distance: {}",
                                    mypos.x,
                                    y,
                                    mypos.x.abs() + y.abs()
                                );
                                break 'outer;
                            }
                            false => {
                                visited.insert((mypos.x, y), "here");
                            }
                        }
                    }
                } else {
                    for y in mypos.y..oldpos.y {
                        match visited.contains_key(&(mypos.x, y)) {
                            true => {
                                println!(
                                    "HQ at ({}, {}), distance: {}",
                                    mypos.x,
                                    y,
                                    mypos.x.abs() + y.abs()
                                );
                                break 'outer;
                            }
                            false => {
                                visited.insert((mypos.x, y), "here");
                            }
                        }
                    }
                }
            }
            (_, 0) => {
                if oldpos.x < mypos.x {
                    for x in oldpos.x + 1..mypos.x {
                        match visited.contains_key(&(x, mypos.y)) {
                            true => {
                                println!(
                                    "HQ at ({}, {}), distance: {}",
                                    x,
                                    mypos.y,
                                    x.abs() + mypos.y.abs()
                                );
                                break 'outer;
                            }
                            false => {
                                visited.insert((x, mypos.y), "here");
                            }
                        }
                    }
                } else {
                    for x in mypos.x..oldpos.x {
                        match visited.contains_key(&(x, mypos.y)) {
                            true => {
                                println!(
                                    "HQ at ({}, {}), distance: {}",
                                    x,
                                    mypos.y,
                                    x.abs() + mypos.y.abs()
                                );
                                break 'outer;
                            }
                            false => {
                                visited.insert((x, mypos.y), "here");
                            }
                        }
                    }
                }
            }
            _ => {
                visited.insert((mypos.x, mypos.y), "here");
            }
        }
        oldpos.x = mypos.x;
        oldpos.y = mypos.y;
    }
}
