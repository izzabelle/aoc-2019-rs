use structopt::StructOpt;

mod days;

#[derive(StructOpt)]
#[structopt(
    name = "Isabelle's Advent of Code",
    about = "I'm actually doing advent of code this year :D"
)]
struct Opt {
    /// advent of code day
    day: String,

    /// use flag for second part of each day
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

        /*3 => {
            if !opt.part {
                days::day_three::part_one();
            } else {
                days::day_three::part_two();
            }
        }*/
        4 => {
            if !opt.part {
                days::day_four::part_one();
            } else {
                days::day_four::part_two();
            }
        }

        5 => {
            if !opt.part {
                days::day_five::part_one();
            } else {
                days::day_five::part_two();
            }
        }

        6 => {
            if !opt.part {
                days::day_six::part_one();
            } else {
                days::day_six::part_two();
            }
        }
        7 => {
            if !opt.part {
                days::day_seven::part_one();
            } else {
                days::day_seven::part_two();
            }
        }

        10 => {
            if !opt.part {
                days::day_ten::part_one();
            } else {
                days::day_ten::part_two();
            }
        }

        99 => {
            advent_of_code::tests::run_all_timed();
        }

        _ => {
            println!("day {} is either not valid or incomplete", day);
        }
    }

    Ok(())
}
