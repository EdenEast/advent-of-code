use std::collections::HashMap;

use libaoc::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Day 01: Historian Hysteria"
    }
    fn part1(&self, input: &str) -> Answer {
        self.part1(input)
    }
    fn part2(&self, input: &str) -> Answer {
        self.part2(input)
    }
}

fn parse_line(line: &str) -> (usize, usize) {
    let (l, r) = line.split_once(|c: char| c.is_whitespace()).unwrap();
    (l.parse().unwrap(), r.trim_ascii_start().parse().unwrap())
}

impl Day01 {
    pub fn part1(&self, input: &str) -> Answer {
        let mut left = Vec::new();
        let mut right = Vec::new();

        input.lines().for_each(|line| {
            let (l, r) = parse_line(line);
            left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
            right.insert(right.binary_search(&r).unwrap_or_else(|e| e), r);
        });

        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<usize>()
            .into()
    }

    pub fn part2(&self, input: &str) -> Answer {
        let mut left = Vec::new();
        let mut right = HashMap::new();
        input.lines().for_each(|line| {
            let (l, r) = parse_line(line);
            left.insert(left.binary_search(&l).unwrap_or_else(|e| e), l);
            *right.entry(r).or_insert(0) += 1;
        });

        left.into_iter()
            .filter_map(|l| right.get(&l).map(|r| l * r))
            .sum::<usize>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../../data/2024/01/test-01");

    #[test]
    fn part_1() {
        assert_eq!(Day01.part1(INPUT), 11.into());
    }
    #[test]
    fn part_2() {
        assert_eq!(Day01.part2(INPUT), 31.into());
    }
}
