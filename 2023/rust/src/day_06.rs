use itertools::Itertools;
use libaoc::{Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Wait For It"
    }

    fn part1(&self, input: &str) -> Answer {
        parse_a(input)
            .iter()
            .map(Race::ways_to_win)
            .product::<u64>()
            .into()
    }

    fn part2(&self, input: &str) -> Answer {
        parse_b(input).ways_to_win().into()
    }
}

#[derive(Debug, Default)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .map(|i| {
                let distance = i * (self.time - i);
                if distance > self.distance {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

fn parse_a(input: &str) -> Vec<Race> {
    let (a, b) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u64>().unwrap())
        })
        .next_tuple()
        .unwrap();

    a.zip(b)
        .map(|(time, distance)| Race { time, distance })
        .collect_vec()
}

fn parse_b(input: &str) -> Race {
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .next_tuple()
        .unwrap();
    Race { time, distance }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day06.part1(include_str!("../../../data/2023/06/test-01"));
        assert_eq!(answer, 288.into());
    }

    #[test]
    fn part_2() {
        let answer = Day06.part2(include_str!("../../../data/2023/06/test-01"));
        assert_eq!(answer, 71503.into());
    }
}
