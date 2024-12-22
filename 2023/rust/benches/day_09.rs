use aoc_2023::day_09::Day09;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day09.part1(divan::black_box(include_str!(
        "../../../data/2023/09/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day09.part2(divan::black_box(include_str!(
        "../../../data/2023/09/input"
    )));
}
