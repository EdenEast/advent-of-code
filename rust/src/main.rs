use std::{str::FromStr, time::Instant};

use args::{Args, Commands};
use clap::Parser;
use common::{human_time, Part, Solution};

mod args;

const DEFAULT_YEAR: u32 = 2023;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run { year, day, part } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let day = day.saturating_sub(1);

            let solutions = get_year(year);
            let solution = match solutions.get(day as usize) {
                Some(s) => s,
                None => {
                    println!("No Solution for day {} in year {}", day, year);
                    return;
                }
            };

            println!("[*] Running: {} ({})", solution.name(), part.to_uppercase());
            let input = common::load(year, day + 1).unwrap();

            let start = Instant::now();
            let out = match part.to_lowercase().to_string().as_str() {
                "a" => solution.part_a(&input),
                "b" => solution.part_b(&input),
                _ => return println!("[-] Invalid Part {part}"),
            };

            let time = start.elapsed().as_nanos();
            println!("[+] OUT: {out} ({})", human_time(time));
        }
        Commands::Test { day, part, year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let day = day.saturating_sub(1);

            let solutions = get_year(year);
            let solution = match solutions.get(day as usize) {
                Some(s) => s,
                None => {
                    println!("No Solution for day {} in year {}", day, year);
                    return;
                }
            };

            let part = match Part::from_str(part.to_lowercase().to_string().as_str()) {
                Ok(part) => part,
                Err(_) => return println!("[-] Invalid Part {part}"),
            };

            println!("[*] Testing: {} ({})", solution.name(), part.as_str());
            let input = common::load_test(year, day + 1, part).unwrap();

            let start = Instant::now();
            let out = match part {
                Part::A => solution.part_a(&input),
                Part::B => solution.part_b(&input),
            };

            let time = start.elapsed().as_nanos();
            println!("[+] OUT: {out} ({})", human_time(time));
        }
        Commands::List { year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = get_year(year);
            println!("[*] Solutions for {year}:");

            for (i, e) in solutions.iter().enumerate().filter(|(_, e)| !e.is_dummy()) {
                println!(
                    " {} Day {}: {}",
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
fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2023 => &aoc_2023::ALL,
        _ => &[],
    }
}
