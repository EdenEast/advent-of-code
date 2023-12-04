use aoc_2023::day_03::Day03;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day03.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/03/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day03.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/03/input"
    )));
}
