use std::collections::HashSet;

use libaoc::{Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Scratchcards"
    }

    fn part1(&self, input: &str) -> Answer {
        parse(input)
            .into_iter()
            .filter(|&wins| wins > 0usize)
            .map(|wins| 2u32.pow(wins.saturating_sub(1) as u32))
            .sum::<u32>()
            .into()
    }

    fn part2(&self, input: &str) -> Answer {
        let cards = parse(input);
        let mut queue = (0..cards.len()).collect::<Vec<_>>();
        let mut total = 0;

        while let Some(index) = queue.pop() {
            total += 1;

            let card = &cards[index];
            if *card == 0 {
                continue;
            }

            for i in 0..*card {
                queue.push(i + index + 1);
            }
        }

        total.into()
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let card = line.split_once(": ").unwrap().1;
            let split = card.split_once('|').unwrap();
            let winning = split
                .0
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let ours = split
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            ours.intersection(&winning).collect::<Vec<_>>().len()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day04.part1(include_str!("../../../data/2023/04/test-01"));
        assert_eq!(answer, 13.into());
    }

    #[test]
    fn part_2() {
        let answer = Day04.part2(include_str!("../../../data/2023/04/test-01"));
        assert_eq!(answer, 30.into());
    }
}
