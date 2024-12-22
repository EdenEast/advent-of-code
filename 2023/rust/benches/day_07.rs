use aoc_2023::day_07::Day07;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day07.part1(divan::black_box(include_str!(
        "../../../data/2023/07/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day07.part2(divan::black_box(include_str!(
        "../../../data/2023/07/input"
    )));
}
