use aoc_2023::day_01::Day01;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1_search_first_and_last() {
    Day01.a_search_first_and_last(divan::black_box(include_str!(
        "../../../data/2023/01/input"
    )));
}

#[divan::bench]
fn part_1_map_to_list() {
    Day01.a_map_to_list(divan::black_box(include_str!(
        "../../../data/2023/01/input"
    )));
}

#[divan::bench]
fn part_2_map_to_list() {
    Day01.b_map_to_list(divan::black_box(include_str!(
        "../../../data/2023/01/input"
    )));
}
