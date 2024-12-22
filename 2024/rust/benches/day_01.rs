use aoc_2024::day_01::Day01;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    Day01.part1(divan::black_box(include_str!(
        "../../../data/2024/01/input"
    )));
}

#[divan::bench]
fn part_2() {
    Day01.part2(divan::black_box(include_str!(
        "../../../data/2024/01/input"
    )));
}
