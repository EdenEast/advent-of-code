use aoc_2024::day_02::Day02;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day02.part1(divan::black_box(include_str!(
        "../../../data/2024/02/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day02.part2(divan::black_box(include_str!(
        "../../../data/2024/02/input"
    )));
}
