fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter_map(|line| {
            // A == 65, X == 88
            let bytes = line.as_bytes();
            Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
        })
        .collect()
}

fn score(theirs: u32, ours: u32) -> u32 {
    // Game results is the wrapped distance. Wrapping with modulo for result (0,1,2) to be
    // (lose, draw, win). This just has to be multiplied by 3 to get (0, 3, 6)
    let score = (3 - (2 + theirs - ours) % 3) % 3 * 3;
    score + ours + 1
}

fn part1(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|(theirs, ours)| score(theirs, ours))
        .sum()
}

fn part2(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|&(theirs, result)| {
            let ours = match result {
                0 => (theirs + 2) % 3,
                1 => theirs,
                2 => (theirs + 1) % 3,
                _ => unreachable!("Unknown result number"),
            };
            score(theirs, ours)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../day/02/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &str = include_str!("../../../day/02/demo.txt");

    #[test]
    fn day02_part1() {
        assert_eq!(15, part1(DEMO));
    }

    #[test]
    fn day02_part2() {
        assert_eq!(12, part2(DEMO));
    }
}
