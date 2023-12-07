use std::collections::HashMap;

const CARD_STRENGTHS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const HAND_TYPE_STRENGTHS: [HandType; 7] = [
    HandType::FiveOfAKind,
    HandType::FourOfAKind,
    HandType::FullHouse,
    HandType::ThreeOfAKind,
    HandType::TwoPair,
    HandType::OnePair,
    HandType::HighCard,
];

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_strength = HAND_TYPE_STRENGTHS.iter().position(|t| t == self).unwrap();
        let other_strength = HAND_TYPE_STRENGTHS.iter().position(|t| t == other).unwrap();

        other_strength.cmp(&this_strength)
    }
}

fn get_counts(cards: &Vec<char>) -> Vec<(char, u32)> {
    let mut counts = HashMap::new();

    for card in cards {
        let entry = counts.entry(*card).or_insert(0);
        *entry += 1;
    }

    counts.into_iter().collect()
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
struct Hand {
    cards: Vec<char>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(&other.cards) {
                    let c1_strength = CARD_STRENGTHS.iter().position(|c| c == c1).unwrap();
                    let c2_strength = CARD_STRENGTHS.iter().position(|c| c == c2).unwrap();

                    match c2_strength.cmp(&c1_strength) {
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

impl Hand {
    fn get_type(&self) -> HandType {
        let counts = get_counts(&self.cards);
        let highest_count = counts.iter().map(|(_, c)| c).max().unwrap();

        match highest_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts.iter().filter(|(_, c)| c == &2).count() == 1 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts.iter().filter(|(_, c)| c == &2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("No cards in hand"),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                Hand {
                    cards: cards.chars().collect(),
                },
                bid.parse().unwrap(),
            )
        })
        .collect()
}

fn process(input: &str) -> u32 {
    let mut hand_bids = parse_input(input);
    hand_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

    hand_bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (1 + i as u32) * b)
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 6440);
    }
}
