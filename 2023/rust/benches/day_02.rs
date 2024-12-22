use aoc_2023::day_02::Day02;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1_match_and_split() {
    Day02.a_match_and_split(divan::black_box(include_str!(
        "../../../data/2023/02/input"
    )));
}

#[divan::bench]
fn part_1_nom_parser() {
    Day02.a_nom_parser(divan::black_box(include_str!(
        "../../../data/2023/02/input"
    )));
}

#[divan::bench]
fn part_2_match_and_split() {
    Day02.b_match_and_split(divan::black_box(include_str!(
        "../../../data/2023/02/input"
    )));
}

#[divan::bench]
fn part_2_nom_parser() {
    Day02.b_nom_parser(divan::black_box(include_str!(
        "../../../data/2023/02/input"
    )));
}
