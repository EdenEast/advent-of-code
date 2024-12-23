use libaoc::{Answer, Solution};

pub struct DayXX;

impl Solution for DayXX {
    fn name(&self) -> &'static str {
        "Insert Name Here"
    }
    fn part1(&self, input: &str) -> Answer {
        self.part1(input)
    }
    fn part2(&self, input: &str) -> Answer {
        self.part2(input)
    }
}

impl DayXX {
    pub fn part1(&self, input: &str) -> Answer {
        0.into()
    }

    pub fn part2(&self, input: &str) -> Answer {
        0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            DayXX.part1(include_str!("../../../data/2024/XX/test-01")),
            -1.into()
        );
    }
    #[test]
    fn part_2() {
        assert_eq!(
            DayXX.part2(include_str!("../../../data/2024/XX/test-01")),
            -1.into()
        );
    }
}
