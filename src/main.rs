use structopt::StructOpt;

mod days;

#[derive(StructOpt)]
#[structopt(name = "Ryan's Advent of Code", about = "Learning Rust with AoC")]
struct Opt {
    /// advent of code day
    day: String,

    /// use flag for secont part of each day
    #[structopt(short = "p", long = "part-two")]
    part: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let day = opt.day.parse::<u8>()?;

    match day {
        1 => {
            if !opt.part {
                days::day_one::part_one();
            } else {
                days::day_one::part_two();
            }
        }

        2 => {
            if !opt.part {
                days::day_two::part_one();
            } else {
                days::day_two::part_two();
            }
        }

        _ => println!("Implement me later!"),
    }
    Ok(())
}
