use aoc_2024::day_XX::DayXX;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    DayXX.part1(divan::black_box(include_str!(
        "../../../data/2024/XX/input"
    )));
}

#[divan::bench]
fn part_2() {
    DayXX.part2(divan::black_box(include_str!(
        "../../../data/2024/XX/input"
    )));
}
