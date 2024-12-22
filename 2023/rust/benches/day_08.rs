use aoc_2023::day_08::Day08;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day08.part1(divan::black_box(include_str!(
        "../../../data/2023/08/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day08.part2(divan::black_box(include_str!(
        "../../../data/2023/08/input"
    )));
}
