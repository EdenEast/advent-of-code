use aoc_2023::day_09::Day09;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day09.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/09/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day09.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/09/input"
    )));
}
