use itertools::Itertools;
use libaoc::{Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Haunted Wasteland"
    }

    /// Start a "AAA" and follow links until "ZZZ" is eventually found
    fn part1(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .map(Sequence::predict)
            .sum::<i64>()
            .into()
    }

    fn part2(&self, input: &str) -> Answer {
        parse(input)
            .iter_mut()
            .map(|x| {
                x.values.reverse();
                x.predict()
            })
            .sum::<i64>()
            .into()
    }
}

struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn differences(&self) -> Vec<Vec<i64>> {
        let mut diffs = vec![self.values.clone()];
        while !diffs.last().unwrap().iter().all(|&x| x == 0) {
            let last = diffs.last().unwrap();
            let mut next = Vec::new();

            for i in 1..last.len() {
                next.push(last[i] - last[i - 1]);
            }
            diffs.push(next);
        }
        diffs
    }

    fn predict(&self) -> i64 {
        self.differences().iter().filter_map(|v| v.last()).sum()
    }
}

fn parse(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| Sequence {
            values: line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect_vec(),
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day09.part1(include_str!("../../../data/2023/09/test-01"));
        assert_eq!(answer, 114.into());
    }

    #[test]
    fn part_2() {
        let answer = Day09.part2(include_str!("../../../data/2023/09/test-01"));
        assert_eq!(answer, 2.into());
    }
}
