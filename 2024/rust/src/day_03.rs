use lazy_static::lazy_static;
use libaoc::{Answer, Solution};

pub struct Day03;

use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"mul\((\d+),(\d+\))|do\(\)|don't\(\)").unwrap();
}

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Mull It Over"
    }
    fn part1(&self, input: &str) -> Answer {
        self.part1(input)
    }
    fn part2(&self, input: &str) -> Answer {
        self.part2(input)
    }
}

impl Day03 {
    pub fn part1(&self, input: &str) -> Answer {
        regex_parse(input, false).into()
    }

    pub fn part2(&self, input: &str) -> Answer {
        regex_parse(input, true).into()
    }
}

fn regex_parse(input: &str, toggle: bool) -> u64 {
    let commands = REGEX.find_iter(input);
    let mut enabled = true;
    let mut result: u64 = 0;

    for command in commands {
        match command.as_str() {
            "do()" => {
                if toggle {
                    enabled = true;
                }
            }
            "don't()" => {
                if toggle {
                    enabled = false;
                }
            }
            s if s.starts_with("mul") => {
                if enabled {
                    let substring = &command.as_str()[4..command.len() - 1];
                    let (a, b) = substring
                        .split_once(',')
                        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
                        .unwrap();

                    result += a * b;
                }
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            Day03.part1(include_str!("../../../data/2024/03/test-01")),
            161.into()
        );
    }
    #[test]
    fn part_2() {
        assert_eq!(
            Day03.part2(include_str!("../../../data/2024/03/test-02")),
            48.into()
        );
    }
}
