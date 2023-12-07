use std::collections::HashMap;

const CARD_STRENGTHS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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
        let jokers = self.cards.iter().filter(|c| c == &&'J').count() as u32;
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }

        let counts = get_counts(&self.cards);
        let (card, highest_count) = counts
            .iter()
            .filter(|(c, _)| c != &'J')
            .max_by(|(_, n1), (_, n2)| n1.cmp(&n2))
            .unwrap();

        match u32::min(5, highest_count + jokers) {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts
                    .iter()
                    .filter(|(c, n)| c != &'J' && c != card && n == &2)
                    .count()
                    == 1
                {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts
                    .iter()
                    .filter(|(c, n)| c != &'J' && c != card && n == &2)
                    .count()
                    == 1
                {
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
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 5905);
    }

    #[test]
    fn test_joker() {
        let hand = Hand {
            cards: "32T3K".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::OnePair);

        let hand = Hand {
            cards: "T55J5".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "KK677".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::TwoPair);

        let hand = Hand {
            cards: "KTJJT".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "QQQJA".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "JJJJJ".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: "JJJJQ".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: "QQQQJ".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: "QQQAJ".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "QQAAJ".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand {
            cards: "QT32J".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::OnePair);

        let hand = Hand {
            cards: "AAA2Q".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: "AAA22".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand {
            cards: "JAA22".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand {
            cards: "JJA12".chars().collect(),
        };
        assert_eq!(hand.get_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_joker_comparison() {
        let hand = Hand {
            cards: "JJJJJ".chars().collect(),
        };

        let other = Hand {
            cards: "QQQQQ".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);

        let hand = Hand {
            cards: "JJJJJ".chars().collect(),
        };

        let other = Hand {
            cards: "JQQQQ".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);

        let hand = Hand {
            cards: "AAJJQ".chars().collect(),
        };

        let other = Hand {
            cards: "QAAJJ".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand {
            cards: "AAJJQ".chars().collect(),
        };

        let other = Hand {
            cards: "QAAAA".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand {
            cards: "AAAAJ".chars().collect(),
        };

        let other = Hand {
            cards: "22222".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand {
            cards: "JAAAA".chars().collect(),
        };

        let other = Hand {
            cards: "22222".chars().collect(),
        };
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);
    }
}
