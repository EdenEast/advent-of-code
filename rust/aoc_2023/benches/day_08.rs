use aoc_2023::day_08::Day08;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day08.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/08/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day08.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/08/input"
    )));
}
