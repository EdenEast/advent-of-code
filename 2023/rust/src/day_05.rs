use libaoc::{Answer, Solution};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "If You Give A Seed A Fertilizer"
    }

    fn part1(&self, input: &str) -> Answer {
        let result = parse(input);

        let mut min = u64::MAX;
        for mut seed in result.seeds {
            for ranges in &result.maps {
                seed = ranges.map(seed);
            }
            min = min.min(seed);
        }

        min.into()
    }

    fn part2(&self, input: &str) -> Answer {
        let result = parse(input);

        result
            .seeds
            .par_chunks_exact(2)
            .map(|seed| {
                let mut min = u64::MAX;
                for mut seed in seed[0]..=seed[0] + seed[1] {
                    for map in &result.maps {
                        seed = map.map(seed);
                    }
                    min = min.min(seed);
                }
                min
            })
            .min()
            .unwrap()
            .into()
    }
}

#[derive(Debug, Default)]
struct Range {
    start: u64,
    end: u64,
    length: u64,
}

#[derive(Debug, Default)]
struct Map(Vec<Range>);

#[derive(Debug, Default)]
struct ParseResult {
    maps: Vec<Map>,
    seeds: Vec<u64>,
}

fn parse(input: &str) -> ParseResult {
    let mut maps = Vec::new();
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // skip 'seeds:'
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for section in sections.filter(|x| !x.is_empty()) {
        let lines = section.lines();
        let mut ranges = Vec::new();

        // Skipping the title of the sction
        for line in lines.skip(1) {
            let mut parts = line.split_whitespace();
            let end = parts.next().unwrap().parse().unwrap();
            let start = parts.next().unwrap().parse().unwrap();
            let length = parts.next().unwrap().parse().unwrap();

            ranges.push(Range { start, end, length });
        }
        maps.push(Map(ranges));
    }

    ParseResult { seeds, maps }
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        for range in &self.0 {
            if range.start <= value && value < range.start + range.length {
                return range.end + value - range.start;
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day05.part1(include_str!("../../../data/2023/05/test-01"));
        assert_eq!(answer, 35.into());
    }

    #[test]
    fn part_2() {
        let answer = Day05.part2(include_str!("../../../data/2023/05/test-01"));
        assert_eq!(answer, 46.into());
    }
}
