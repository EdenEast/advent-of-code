use std::fmt::Display;

use itertools::Itertools;
use libaoc::{Answer, Solution};

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(&parse::<part1::Card>(input)).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(&parse::<part2::Card>(input)).into()
    }
}

fn parse<T>(input: &str) -> Vec<Hand<T>>
where
    T: Ord + Copy + TryFrom<char>,
    HandType: From<([T; 5])>,
{
    input
        .lines()
        .map(|line| {
            let (raw, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let cards: [T; 5] = raw
                .chars()
                .map(|card| {
                    card.try_into()
                        .unwrap_or_else(|_| panic!("Invalid card: {card}"))
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid hand: {raw}"));
            let hand_type = cards.into();
            Hand {
                raw,
                cards,
                hand_type,
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

fn solve<T: Ord + Copy>(input: &[Hand<T>]) -> u64 {
    input
        .iter()
        .sorted_unstable_by(|a, b| a.cmp(b))
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// For debugging
impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::HighCard => write!(f, "Hicard"),
            HandType::OnePair => write!(f, "OnePair"),
            HandType::TwoPairs => write!(f, "TwoPair"),
            HandType::ThreeOfAKind => write!(f, "ThreeOfAKind"),
            HandType::FullHouse => write!(f, "FullHouse"),
            HandType::FourOfAKind => write!(f, "FourOfAKind"),
            HandType::FiveOfAKind => write!(f, "FiveOfAKind"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand<'a, T: Ord> {
    raw: &'a str,
    cards: [T; 5],
    bid: u64,
    hand_type: HandType,
}

impl<'a, T: Ord> Ord for Hand<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => {}
                        other => return other,
                    }
                }
                std::cmp::Ordering::Equal
            }
            other => other,
        }
    }
}

mod part1 {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum Card {
        Two = 0,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        T,
        J,
        Q,
        K,
        A,
    }

    impl Card {
        fn iter() -> impl Iterator<Item = Card> {
            use Card::*;
            [
                Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A,
            ]
            .iter()
            .copied()
        }
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Self::Two),
                '3' => Ok(Self::Three),
                '4' => Ok(Self::Four),
                '5' => Ok(Self::Five),
                '6' => Ok(Self::Six),
                '7' => Ok(Self::Seven),
                '8' => Ok(Self::Eight),
                '9' => Ok(Self::Nine),
                'T' => Ok(Self::T),
                'J' => Ok(Self::J),
                'Q' => Ok(Self::Q),
                'K' => Ok(Self::K),
                'A' => Ok(Self::A),
                _ => Err("Invalid card"),
            }
        }
    }

    impl From<[Card; 5]> for HandType {
        fn from(value: [Card; 5]) -> Self {
            let mut counts = [0u8; 13];
            for c in value {
                counts[c as usize] += 1;
            }
            let counts: (u8, u8, u8, u8, u8) = counts
                .into_iter()
                .sorted_unstable_by(|a, b| b.cmp(a))
                .take(5)
                .collect_tuple()
                .unwrap();

            match counts {
                (5, _, _, _, _) => HandType::FiveOfAKind,
                (4, _, _, _, _) => HandType::FourOfAKind,
                (3, 2, _, _, _) => HandType::FullHouse,
                (3, _, _, _, _) => HandType::ThreeOfAKind,
                (2, 2, _, _, _) => HandType::TwoPairs,
                (2, _, _, _, _) => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }
}

mod part2 {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum Card {
        J = 0,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        T,
        Q,
        K,
        A,
    }

    impl Card {
        fn iter() -> impl Iterator<Item = Card> {
            use Card::*;
            [
                J, Two, Three, Four, Five, Six, Seven, Eight, Nine, T, Q, K, A,
            ]
            .iter()
            .copied()
        }
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Self::Two),
                '3' => Ok(Self::Three),
                '4' => Ok(Self::Four),
                '5' => Ok(Self::Five),
                '6' => Ok(Self::Six),
                '7' => Ok(Self::Seven),
                '8' => Ok(Self::Eight),
                '9' => Ok(Self::Nine),
                'T' => Ok(Self::T),
                'J' => Ok(Self::J),
                'Q' => Ok(Self::Q),
                'K' => Ok(Self::K),
                'A' => Ok(Self::A),
                _ => Err("Invalid card"),
            }
        }
    }

    impl ToString for Card {
        fn to_string(&self) -> String {
            match self {
                Card::J => "J".to_string(),
                Card::Two => "2".to_string(),
                Card::Three => "3".to_string(),
                Card::Four => "4".to_string(),
                Card::Five => "5".to_string(),
                Card::Six => "6".to_string(),
                Card::Seven => "7".to_string(),
                Card::Eight => "8".to_string(),
                Card::Nine => "9".to_string(),
                Card::T => "T".to_string(),
                Card::Q => "Q".to_string(),
                Card::K => "K".to_string(),
                Card::A => "A".to_string(),
            }
        }
    }

    impl From<[Card; 5]> for HandType {
        fn from(value: [Card; 5]) -> Self {
            let mut counts = [0u8; 13];
            for c in value {
                counts[c as usize] += 1;
            }

            // Get the number of jokers and then clear that count
            let jokers = counts[Card::J as usize];
            counts[Card::J as usize] = 0;

            let counts = counts
                .into_iter()
                .filter(|&x| x != 0)
                .sorted_unstable_by(|a, b| b.cmp(a))
                .collect_vec();

            if counts.len() <= 1 || counts[0] + jokers == 5 {
                HandType::FiveOfAKind
            } else if counts[0] + jokers == 4 {
                HandType::FourOfAKind
            } else if ((counts[0] + jokers == 3) && counts[1] == 2)
                || (counts[0] == 3 && (counts[1] + jokers == 2))
            {
                HandType::FullHouse
            } else if counts[0] + jokers == 3 {
                HandType::ThreeOfAKind
            } else if ((counts[0] + jokers == 2) && counts[1] == 2)
                || (counts[0] == 2 && (counts[1] + jokers == 2))
            {
                HandType::TwoPairs
            } else if counts[0] + jokers == 2 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = Day07.part1(include_str!("../../../data/2023/07/test-01"));
        assert_eq!(answer, 6440.into());
    }

    #[test]
    fn part_2() {
        let answer = Day07.part2(include_str!("../../../data/2023/07/test-01"));
        assert_eq!(answer, 5905.into());
    }
}
