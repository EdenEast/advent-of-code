use aoc_2023::day_06::Day06;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day06.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/06/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day06.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/06/input"
    )));
}
