use aoc_2024::day_03::Day03;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day03.part1(divan::black_box(include_str!(
        "../../../data/2024/03/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day03.part2(divan::black_box(include_str!(
        "../../../data/2024/03/input"
    )));
}
