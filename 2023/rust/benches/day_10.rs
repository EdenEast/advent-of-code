use aoc_2023::day_10::Day10;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day10.part1(divan::black_box(include_str!(
        "../../../data/2023/10/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day10.part2(divan::black_box(include_str!(
        "../../../data/2023/10/input"
    )));
}
