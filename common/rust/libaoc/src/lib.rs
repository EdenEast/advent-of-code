mod answer;
mod solution;

pub use answer::Answer;
pub use solution::{DummySolution, Solution};

pub fn load(year: u32, day: u32) -> std::io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

pub fn load_raw(year: u32, day: u32) -> std::io::Result<String> {
    let file = format!("../../../data/{year}/{:02}/input", day);
    std::fs::read_to_string(file)
}

pub fn load_test(year: u32, day: u32, part: u32) -> std::io::Result<String> {
    if let Ok(content) = load_raw_test(year, day, part) {
        return Ok(content.trim().replace('\r', ""));
    }
    load_raw_test(year, day, 3 - part).map(|x| x.trim().replace('\r', ""))
}

pub fn load_raw_test(year: u32, day: u32, part: u32) -> std::io::Result<String> {
    let file = format!("../../../data/{year}/{:02}/test-{:02}", day, part);
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
