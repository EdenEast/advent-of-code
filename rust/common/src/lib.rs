#![allow(unused)]

mod answer;
mod solution;

pub use answer::Answer;
pub use solution::Solution;

pub fn load(year: u32, day: u32) -> std::io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

pub fn load_raw(year: u32, day: u32) -> std::io::Result<String> {
    let file = format!("../puzzles/{year}/{:02}/input", day);
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
