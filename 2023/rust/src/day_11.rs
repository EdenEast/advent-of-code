use std::collections::HashSet;

use itertools::Itertools;
use libaoc::{Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Cosmic Expand"
    }

    fn part1(&self, input: &str) -> Answer {
        Galaxy::new(input, 2).total_distance().into()
    }

    fn part2(&self, input: &str) -> Answer {
        Galaxy::new(input, 1_000_000).total_distance().into()
    }
}

type Position = (usize, usize);

#[derive(Debug)]
struct Galaxy {
    galaxies: Vec<Position>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    expand_factor: usize,
}

impl Galaxy {
    fn new(input: &str, expand_factor: usize) -> Self {
        let mut rows = HashSet::new();
        let mut cols = HashSet::new();
        let mut galaxies = Vec::new();

        let grid = input
            .lines()
            .enumerate()
            .map(|(r, row)| {
                row.chars()
                    .enumerate()
                    .map(|(c, ch)| {
                        if ch == '#' {
                            galaxies.push((r, c));
                            rows.insert(r);
                            cols.insert(c);
                            true
                        } else {
                            false
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        let nrows = grid.len();
        let ncols = grid[0].len();

        let mut offset = 0;
        let mut r_offset = Vec::with_capacity(nrows);
        for i in 0..nrows {
            if !rows.contains(&i) {
                offset += 1;
            }
            r_offset.push(offset)
        }

        offset = 0;
        let mut c_offset = Vec::with_capacity(nrows);
        for i in 0..ncols {
            if !cols.contains(&i) {
                offset += 1;
            }
            c_offset.push(offset)
        }

        Self {
            galaxies,
            rows: r_offset,
            cols: c_offset,
            expand_factor: expand_factor - 1,
        }
    }

    fn expand_position(&self, pos: Position) -> (usize, usize) {
        let offset = (self.rows[pos.0], self.cols[pos.1]);
        (
            offset.0 * self.expand_factor + pos.0,
            offset.1 * self.expand_factor + pos.1,
        )
    }

    fn total_distance(&self) -> usize {
        let len = self.galaxies.len();
        let mut total = 0;
        for i in 0..len {
            for j in i + 1..len {
                total += distance(
                    self.expand_position(self.galaxies[i]),
                    self.expand_position(self.galaxies[j]),
                )
            }
        }
        total
    }
}

/// Manhattan distance
fn distance(a: Position, b: Position) -> usize {
    b.0.abs_diff(a.0) + b.1.abs_diff(a.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day11.part1(include_str!("../../../data/2023/11/test-01"));
        assert_eq!(answer, 374.into());
    }

    #[test]
    fn part_2() {
        let answer = Day11.part2(include_str!("../../../data/2023/11/test-01"));
        assert_eq!(answer, 82000210.into());
    }
}
