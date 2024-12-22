use std::collections::HashMap;

use libaoc::{Answer, Solution};
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Gear Ratios"
    }

    fn part1(&self, input: &str) -> Answer {
        parse(input).gears.iter().sum::<u32>().into()
    }

    fn part2(&self, input: &str) -> Answer {
        parse(input)
            .ratios
            .iter()
            .filter(|(_, gears)| gears.len() == 2)
            .map(|(_, gears)| gears[0] * gears[1])
            .sum::<u32>()
            .into()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Span(usize, usize);

struct ParseResult {
    pub gears: Vec<u32>,
    pub ratios: HashMap<Span, Vec<u32>>, // A ratio is a '*' and contains a list of gears
}

fn parse(input: &str) -> ParseResult {
    // Store a list of symbols. This can be used later to check when we have parsed a number if
    // any neighbour is a symbol
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert(Span(x, y), c);
            }
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let mut gears = Vec::new();
    let mut ratios = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for m in regex.find_iter(line) {
            let value = m.as_str().parse().unwrap();

            let mut is_part_number = false;
            for nx in m.start().saturating_sub(1)..=m.end() {
                for ny in y.saturating_sub(1)..=y + 1 {
                    let span = Span(nx, ny);
                    let symbol = symbols.get(&span);
                    is_part_number |= symbol.is_some();

                    if symbol == Some(&'*') {
                        ratios.entry(span).or_insert(Vec::new()).push(value);
                    }
                }
            }

            if is_part_number {
                gears.push(value);
            }
        }
    }

    ParseResult { gears, ratios }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day03.part1(include_str!("../../../data/2023/03/test-01"));
        assert_eq!(answer, 4361.into());
    }

    #[test]
    fn part_2() {
        let answer = Day03.part2(include_str!("../../../data/2023/03/test-01"));
        assert_eq!(answer, 467835.into());
    }
}
