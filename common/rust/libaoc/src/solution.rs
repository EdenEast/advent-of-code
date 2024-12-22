use crate::Answer;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part1(&self, input: &str) -> Answer;
    fn part2(&self, input: &str) -> Answer;

    fn is_dummy(&self) -> bool {
        false
    }
}

pub struct DummySolution;

impl Solution for DummySolution {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn part1(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn part2(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn is_dummy(&self) -> bool {
        true
    }
}
