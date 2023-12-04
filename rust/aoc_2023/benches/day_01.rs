use aoc_2023::day_01::Day01;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1_search_first_and_last() {
    let d = Day01;
    d.a_search_first_and_last(divan::black_box(include_str!(
        "../../../puzzles/2023/01/input"
    )));
}

#[divan::bench]
fn part_1_map_to_list() {
    let d = Day01;
    d.a_map_to_list(divan::black_box(include_str!(
        "../../../puzzles/2023/01/input"
    )));
}

#[divan::bench]
fn part_2_map_to_list() {
    let d = Day01;
    d.b_map_to_list(divan::black_box(include_str!(
        "../../../puzzles/2023/01/input"
    )));
}
