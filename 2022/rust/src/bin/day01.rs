fn collect_elves(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    collect_elves(input).into_iter().max().unwrap_or(0)
}

fn part2(input: &str) -> u32 {
    let mut elves = collect_elves(&input);
    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
    elves.into_iter().take(3).sum()
}

fn main() {
    let input = include_str!("../../../day/01/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &str = include_str!("../../../day/01/demo.txt");

    #[test]
    fn demo_part1() {
        assert_eq!(24000, part1(DEMO));
    }

    #[test]
    fn demo_part2() {
        assert_eq!(45000, part2(DEMO));
    }
}
