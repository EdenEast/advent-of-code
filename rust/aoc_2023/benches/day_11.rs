use aoc_2023::day_11::Day11;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day11.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/11/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day11.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/11/input"
    )));
}
