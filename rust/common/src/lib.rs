#![allow(unused)]

mod answer;
mod solution;

use std::{char::ParseCharError, str::FromStr, string::ParseError};

pub use answer::Answer;
pub use solution::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    A,
    B,
}

impl Part {
    pub fn as_str(&self) -> &'static str {
        match self {
            Part::A => "A",
            Part::B => "B",
        }
    }

    fn as_num(&self) -> u8 {
        match self {
            Part::A => 1,
            Part::B => 2,
        }
    }

    fn other(&self) -> Part {
        match self {
            Part::A => Part::B,
            Part::B => Part::A,
        }
    }
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Part::A),
            "b" => Ok(Part::B),
            _ => Err(format!("Unkonwn part {s}")),
        }
    }
}

pub fn load(year: u32, day: u32) -> std::io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

pub fn load_raw(year: u32, day: u32) -> std::io::Result<String> {
    let file = format!("../puzzles/{year}/{:02}/input", day);
    std::fs::read_to_string(file)
}

pub fn load_test(year: u32, day: u32, part: Part) -> std::io::Result<String> {
    if let Ok(content) = load_raw_test(year, day, part) {
        return Ok(content.trim().replace('\r', ""));
    }
    load_raw_test(year, day, part.other()).map(|x| x.trim().replace('\r', ""))
}

pub fn load_raw_test(year: u32, day: u32, part: Part) -> std::io::Result<String> {
    let file = format!("../puzzles/{year}/{:02}/test-{:02}", day, part.as_num());
    std::fs::read_to_string(file)
}

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
