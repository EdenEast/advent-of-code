use common::{Answer, Solution};

pub struct Day02;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const MAX_CUBES: [u32; 3] = [12, 13, 14];

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part_a(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .enumerate()
            .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
            .map(|x| x.0 + 1)
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .map(|games| {
                let max = games.iter().fold(CubeSet::default(), |acc, x| acc.max(x));
                max.red * max.green * max.blue
            })
            .sum::<u32>()
            .into()
    }
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    input
        .lines()
        .map(|line| {
            let cubes = line
                .split_once(":")
                .expect("There has to be a ':' in game format")
                .1;

            let mut set = Vec::new();
            for game in cubes.split(';') {
                let mut cube_set = CubeSet::default();
                for i in game.split(',') {
                    let mut it = i.split_whitespace();
                    let count = it.next().unwrap().parse::<u32>().unwrap();
                    let color = it.next().unwrap();
                    match color {
                        "red" => cube_set.red += count,
                        "green" => cube_set.green += count,
                        "blue" => cube_set.blue += count,
                        _ => unreachable!(),
                    }
                }
                set.push(cube_set);
            }
            set
        })
        .collect()
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a() {
        let answer = Day02.part_a(include_str!("../../../puzzles/2023/02/test-01"));
        assert_eq!(answer, 8.into());
    }

    #[test]
    fn part_b() {
        let answer = Day02.part_b(include_str!("../../../puzzles/2023/02/test-01"));
        assert_eq!(answer, 2286.into());
    }
}
