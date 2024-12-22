use libaoc::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Red-Nosed Reports"
    }
    fn part1(&self, input: &str) -> Answer {
        self.part1(input)
    }
    fn part2(&self, input: &str) -> Answer {
        self.part2(input)
    }
}

impl Day02 {
    pub fn part1(&self, input: &str) -> Answer {
        input
            .lines()
            .filter(|line| validate(&parse_line(line)))
            .count()
            .into()
    }

    pub fn part2(&self, input: &str) -> Answer {
        input
            .lines()
            .filter(|line| {
                let numbers = parse_line(line);
                validate(&numbers)
                    || (0..numbers.len()).any(|i| {
                        let mut numbers = numbers.clone();
                        numbers.remove(i);
                        validate(&numbers)
                    })
            })
            .count()
            .into()
    }
}

fn parse_line(line: &str) -> Vec<u8> {
    line.split_ascii_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

fn validate(values: &Vec<u8>) -> bool {
    let mut inc = true;
    let mut dec = true;

    for win in values.windows(2) {
        if win[0].abs_diff(win[1]) > 3 {
            return false;
        }

        inc &= win[0] < win[1];
        dec &= win[0] > win[1];
    }
    inc ^ dec
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../../data/2024/02/test-01");

    #[test]
    fn part_1() {
        assert_eq!(Day02.part1(INPUT), 2.into());
    }
    #[test]
    fn part_2() {
        assert_eq!(Day02.part2(INPUT), 4.into());
    }
}
