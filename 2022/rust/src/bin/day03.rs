use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let common = first.chars().find(|c| second.contains(*c)).unwrap();
            *scores.get(&common).unwrap() as u32
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    let mut lines = input.lines();
    let nlines= lines.clone().count();
    let iterations = nlines / 3;

    (0..iterations).map(|_|{
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let common = first.chars().find(|c| second.contains(*c) && third.contains(*c) ).unwrap();
        *scores.get(&common).unwrap() as u32

    }).sum()
}

fn main() {
    let input = include_str!("../../../day/03/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &str = include_str!("../../../day/03/demo.txt");

    #[test]
    fn day03_part1() {
        assert_eq!(157, part1(DEMO));
    }

    #[test]
    fn day03_part2() {
        assert_eq!(70, part2(DEMO));
    }
}
