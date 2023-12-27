use aoc_2023::day_10::Day10;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day10.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/10/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day10.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/10/input"
    )));
}
