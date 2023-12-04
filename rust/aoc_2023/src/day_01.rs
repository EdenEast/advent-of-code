use common::{Answer, Solution};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Trebuchet?!"
    }

    fn part_a(&self, input: &str) -> Answer {
        self.a_search_first_and_last(input)
    }

    fn part_b(&self, input: &str) -> Answer {
        self.b_map_to_list(input)
    }
}

// Part 1 Solutions
impl Day01 {
    pub fn a_search_first_and_last(&self, input: &str) -> Answer {
        input
            .lines()
            .map(|line| {
                let mut it = line.chars();
                let first = it
                    .find_map(|c| c.to_digit(10))
                    .expect("There should be a number");
                let last = it.rev().find_map(|c| c.to_digit(10)).unwrap_or(first);
                first * 10 + last
            })
            .sum::<u32>()
            .into()
    }

    pub fn a_map_to_list(&self, input: &str) -> Answer {
        input
            .lines()
            .map(|line| {
                let mut digits = line.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().expect("There should be a number");
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .sum::<u32>()
            .into()
    }

    pub fn b_map_to_list(&self, input: &str) -> Answer {
        input
            .lines()
            .map(|line| {
                let mut numbers = (0..line.len()).filter_map(|index| {
                    let window = &line[index..];
                    if let Some((i, _)) = DIGITS
                        .iter()
                        .enumerate()
                        .find(|(_, digit)| window.starts_with(*digit))
                    {
                        Some((i + 1) as u32)
                    } else {
                        // Check the char at index for digit
                        window.chars().next().and_then(|c| c.to_digit(10))
                    }
                });

                let first = numbers.next().expect("There should be a number");
                let last = numbers.last().unwrap_or(first);
                first * 10 + last
            })
            .sum::<u32>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_search_first_and_last() {
        let d = Day01;
        let answer = d.a_search_first_and_last(include_str!("../../../puzzles/2023/01/test-01"));
        assert_eq!(answer, Answer::Number(142));
    }

    #[test]
    fn part_1_map_to_list() {
        let d = Day01;
        let answer = d.a_map_to_list(include_str!("../../../puzzles/2023/01/test-01"));
        assert_eq!(answer, Answer::Number(142));
    }

    #[test]
    fn part_2_map_to_list() {
        let d = Day01;
        let answer = d.b_map_to_list(include_str!("../../../puzzles/2023/01/test-02"));
        assert_eq!(answer, Answer::Number(281));
    }
}
