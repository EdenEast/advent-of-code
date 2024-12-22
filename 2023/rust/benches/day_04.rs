use aoc_2023::day_04::Day04;
use libaoc::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day04.part1(divan::black_box(include_str!(
        "../../../data/2023/04/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day04.part2(divan::black_box(include_str!(
        "../../../data/2023/04/input"
    )));
}
