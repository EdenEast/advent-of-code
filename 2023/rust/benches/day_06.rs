use aoc_2023::day_06::Day06;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day06.part1(divan::black_box(include_str!(
        "../../../data/2023/06/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day06.part2(divan::black_box(include_str!(
        "../../../data/2023/06/input"
    )));
}
