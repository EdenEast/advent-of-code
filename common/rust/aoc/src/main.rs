use std::time::Instant;

use clap::{command, Parser, Subcommand};
use eyre::eyre;
use libaoc::Solution;

const DEFAULT_YEAR: u32 = 2024;
const YEARS: [u32; 2] = [2024, 2023];

#[derive(Parser)]
#[command(name = "advent_of_code")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, help = "Prints the answer only")]
    pub minimal: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias("r"), about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: u32,
        year: Option<u32>,
    },
    #[command(visible_alias("t"), about = "Run the test input to a problem")]
    Test {
        day: u32,
        part: u32,
        year: Option<u32>,
    },
    #[command(visible_alias("l"), about = "List all solutions for a given year")]
    List { year: Option<u32> },
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { day, part, year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let day = day.saturating_sub(1);

            let solutions = get_year(year);
            let solution = solutions
                .get(day as usize)
                .ok_or_else(|| eyre!("No solution found for day {} of year {}", day + 1, year))?;

            let input = libaoc::load(year, day + 1).unwrap();

            if !args.minimal {
                println!("[*] Running: {} ({})", solution.name(), part);
            }
            let start = Instant::now();
            let out = match part {
                1 => solution.part1(&input),
                2 => solution.part2(&input),
                _ => {
                    return Err(eyre!("Invalid part number: {}", part).into());
                }
            };

            let time = start.elapsed().as_nanos();
            if args.minimal {
                println!("{}", out);
            } else {
                println!("[+] OUT: {out} ({})", libaoc::human_time(time));
            }
        }
        Commands::Test { day, part, year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let day = day.saturating_sub(1);

            let solutions = get_year(year);
            let solution = solutions
                .get(day as usize)
                .ok_or_else(|| eyre!("No solution found for day {} of year {}", day + 1, year))?;

            let input = libaoc::load_test(year, day + 1, part).unwrap();

            if !args.minimal {
                println!("[*] Testing: {} ({})", solution.name(), part);
            }
            let start = Instant::now();
            let out = match part {
                1 => solution.part1(&input),
                2 => solution.part2(&input),
                _ => {
                    return Err(eyre!("Invalid part number: {}", part).into());
                }
            };

            let time = start.elapsed().as_nanos();
            if args.minimal {
                println!("{}", out);
            } else {
                println!("[+] OUT: {out} ({})", libaoc::human_time(time));
            }
        }
        Commands::List { year } => {
            let years = year.map(|y| vec![y]).unwrap_or_else(|| YEARS.to_vec());
            for y in years {
                let solutions = get_year(y);
                println!("[*] Solutions for {y}:");

                for (i, e) in solutions.iter().enumerate().filter(|(_, e)| !e.is_dummy()) {
                    println!(
                        " {} Day {:02}: {}",
                        if i + 1 == solutions.len() {
                            "└"
                        } else {
                            "├"
                        },
                        i + 1,
                        e.name()
                    );
                }
            }
        }
    }

    Ok(())
}

fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2024 => &aoc_2024::ALL,
        2023 => &aoc_2023::ALL,
        _ => &[],
    }
}
