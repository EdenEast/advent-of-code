use std::{
    collections::{hash_set, BTreeMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;
use libaoc::{Answer, Solution};

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Pipe Maze"
    }

    fn part1(&self, input: &str) -> Answer {
        Maze::new(input).find_path().len().div_ceil(2).into()
    }

    // Not sure how this solution works found an interesting solution here:
    // https://github.com/ephemient/aoc2023/blob/11a51bb7fa1e4485256d9840d5e9435f573e062d/rs/src/day10.rs#L69-L77
    fn part2(&self, input: &str) -> Answer {
        let path = Maze::new(input).find_path();
        let (n, m) = path
            .iter()
            .zip(path[1..].iter().chain(path.iter()))
            .map(|((r0, c0), (r1, c1))| (c0 * r1, c1 * r0))
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
        ((2 + n.max(m) - n.min(m) - path.len()) / 2).into()
    }
}

type Cell = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn starting(c: char) -> Self {
        if c == '┌' || c == '└' {
            Self::Right
        } else {
            Self::Left
        }
    }

    fn apply(&self, cell: Cell) -> Cell {
        match self {
            Direction::Up => (cell.0.saturating_sub(1), cell.1),
            Direction::Down => (cell.0 + 1, cell.1),
            Direction::Left => (cell.0, cell.1.saturating_sub(1)),
            Direction::Right => (cell.0, cell.1 + 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

trait DirectionExt {
    fn is_up(&self) -> bool;
    fn is_down(&self) -> bool;
    fn is_left(&self) -> bool;
    fn is_right(&self) -> bool;
    fn convert(&self) -> char;
}

impl DirectionExt for char {
    fn is_up(&self) -> bool {
        matches!(self, '|' | 'F' | '7' | '┌' | '┐')
    }

    fn is_down(&self) -> bool {
        matches!(self, '|' | 'L' | 'J' | '└' | '┘')
    }

    fn is_left(&self) -> bool {
        matches!(self, '-' | 'L' | 'F' | '└' | '┌')
    }

    fn is_right(&self) -> bool {
        matches!(self, '-' | '7' | 'J' | '┐' | '┘')
    }

    fn convert(&self) -> char {
        match self {
            'L' => '└',
            'J' => '┘',
            '7' => '┐',
            'F' => '┌',
            c => *c,
        }
    }
}

struct Maze {
    start: Cell,
    grid: Vec<Vec<char>>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.convert()).collect_vec())
            .collect_vec();

        let start = grid
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
            })
            .unwrap();
        let right = grid[start.0][start.1 + 1];
        let down = grid[start.0 + 1][start.1];

        let c = match (right.is_right(), down.is_down()) {
            (true, true) => '┌',
            (true, false) => '└',
            (false, true) => '┐',
            (false, false) => '┘',
        };

        grid[start.0][start.1] = c;

        Self { start, grid }
    }

    fn find_path(&self) -> Vec<Cell> {
        let mut segments = Vec::new();
        let mut direction = match self.grid[self.start.0][self.start.1] {
            '┌' | '└' => Direction::Right,
            _ => Direction::Left,
        };

        let mut pos = direction.apply(self.start);
        segments.push(self.start);
        while pos != self.start {
            segments.push(pos);

            let c = self.grid[pos.0][pos.1];
            // NOTE: The direction we are matching against is the incoming direction not
            // the direction that it could be. For example '┌', character can come from
            // either Up or Left (not the directions that it points to ie down and right)
            direction = match (c, direction) {
                ('┌', Direction::Up) => Direction::Right,
                ('┌', Direction::Left) => Direction::Down,
                ('└', Direction::Down) => Direction::Right,
                ('└', Direction::Left) => Direction::Up,
                ('┐', Direction::Up) => Direction::Left,
                ('┐', Direction::Right) => Direction::Down,
                ('┘', Direction::Down) => Direction::Left,
                ('┘', Direction::Right) => Direction::Up,
                ('|', Direction::Up) => Direction::Up,
                ('|', Direction::Down) => Direction::Down,
                ('-', Direction::Left) => Direction::Left,
                ('-', Direction::Right) => Direction::Right,
                c => panic!("Unknown character {} in direction: {}", c.0, c.1),
            };

            pos = direction.apply(pos);
        }

        segments
    }

    fn print(&self) {
        for (r, row) in self.grid.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                print!("{ch}");
            }
            println!("");
        }
    }

    fn print_loop(&self, segments: &HashSet<Cell>) {
        for (r, row) in self.grid.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                if segments.contains(&(r, c)) {
                    print!("{ch}");
                } else {
                    // print!("█");
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;
    #[rstest]
    #[case(indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "}, 4.into())]
    #[case(indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "}, 8.into())]
    fn part_1(#[case] input: &str, #[case] expected: Answer) {
        assert_eq!(expected, Day10.part1(input))
    }

    #[rstest]
    #[case(indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "}, 4.into())]
    #[case(indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "} , 10.into())]
    fn part_2(#[case] input: &str, #[case] expected: Answer) {
        assert_eq!(expected, Day10.part2(input))
    }
}
