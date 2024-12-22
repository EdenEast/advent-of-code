use libaoc::{Answer, Solution};

mod nom;

pub struct Day02;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const MAX_CUBES: [u32; 3] = [12, 13, 14];

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part1(&self, input: &str) -> Answer {
        self.a_nom_parser(input)
    }

    fn part2(&self, input: &str) -> Answer {
        self.b_nom_parser(input)
    }
}

impl Day02 {
    pub fn a_match_and_split(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .enumerate()
            .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
            .map(|x| x.0 + 1)
            .sum::<usize>()
            .into()
    }

    pub fn a_nom_parser(&self, input: &str) -> Answer {
        nom::parse(input)
            .unwrap()
            .1
            .iter()
            .enumerate()
            .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
            .map(|x| x.0 + 1)
            .sum::<usize>()
            .into()
    }

    pub fn b_match_and_split(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .map(|games| {
                let max = games.iter().fold(CubeSet::default(), |acc, x| acc.max(x));
                max.red * max.green * max.blue
            })
            .sum::<u32>()
            .into()
    }

    pub fn b_nom_parser(&self, input: &str) -> Answer {
        nom::parse(input)
            .unwrap()
            .1
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
pub struct CubeSet {
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
    fn part1_match_and_split() {
        let answer = Day02.a_match_and_split(include_str!("../../../../data/2023/02/test-01"));
        assert_eq!(answer, 8.into());
    }

    #[test]
    fn part1_nom_parser() {
        let answer = Day02.a_nom_parser(include_str!("../../../../data/2023/02/test-01"));
        assert_eq!(answer, 8.into());
    }

    #[test]
    fn part2_match_and_split() {
        let answer = Day02.b_match_and_split(include_str!("../../../../data/2023/02/test-01"));
        assert_eq!(answer, 2286.into());
    }

    #[test]
    fn part2_nom_parser() {
        let answer = Day02.b_nom_parser(include_str!("../../../../data/2023/02/test-01"));
        assert_eq!(answer, 2286.into());
    }
}
