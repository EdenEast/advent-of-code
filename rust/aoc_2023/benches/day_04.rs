use aoc_2023::day_04::Day04;
use common::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day04.part_a(divan::black_box(include_str!(
        "../../../puzzles/2023/04/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day04.part_b(divan::black_box(include_str!(
        "../../../puzzles/2023/04/input"
    )));
}
