use aoc_2023::day_11::Day11;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day11.part1(divan::black_box(include_str!(
        "../../../data/2023/11/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day11.part2(divan::black_box(include_str!(
        "../../../data/2023/11/input"
    )));
}
