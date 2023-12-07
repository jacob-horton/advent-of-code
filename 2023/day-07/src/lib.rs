use std::collections::HashMap;

// Index 0 = strongest
const CARD_STRENGTHS_BASIC: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

// Index 0 = strongest
const CARD_STRENGTHS_JOKER_WILDCARD: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

// Index 0 = strongest
pub const HAND_TYPE_STRENGTHS: [HandType; 7] = [
    HandType::FiveOfAKind,
    HandType::FourOfAKind,
    HandType::FullHouse,
    HandType::ThreeOfAKind,
    HandType::TwoPair,
    HandType::OnePair,
    HandType::HighCard,
];

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum HandType {
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

pub fn get_counts(cards: &Vec<char>) -> Vec<(char, u32)> {
    let mut counts = HashMap::new();

    for card in cards {
        let entry = counts.entry(*card).or_insert(0);
        *entry += 1;
    }

    counts.into_iter().collect()
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Hand {
    cards: Vec<char>,
    jokers_as_wildcard: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let card_strengths = if self.jokers_as_wildcard {
            &CARD_STRENGTHS_JOKER_WILDCARD
        } else {
            &CARD_STRENGTHS_BASIC
        };

        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(&other.cards) {
                    let c1_strength = card_strengths.iter().position(|c| c == c1).unwrap();
                    let c2_strength = card_strengths.iter().position(|c| c == c2).unwrap();

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
    pub fn new(cards: Vec<char>, jokers_as_wildcard: bool) -> Self {
        Self {
            cards,
            jokers_as_wildcard,
        }
    }

    pub fn get_type(&self) -> HandType {
        let wildcards = match self.jokers_as_wildcard {
            true => self.cards.iter().filter(|c| c == &&'J').count() as u32,
            false => 0,
        };

        if wildcards == 5 {
            return HandType::FiveOfAKind;
        }

        let counts = get_counts(&self.cards);
        let (char, highest_count) = counts
            .iter()
            .filter(|(c, _)| c != &'J')
            .max_by(|(_, n1), (_, n2)| n1.cmp(n2))
            .unwrap();

        match highest_count + wildcards {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts
                    .iter()
                    .filter(|(c, n)| c != &'J' && c != char && n == &2)
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
                    .filter(|(c, n)| c != &'J' && c != char && n == &2)
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

pub fn parse_input(input: &str, jokers_as_wildcard: bool) -> Vec<(Hand, u32)> {
    input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                Hand::new(cards.chars().collect(), jokers_as_wildcard),
                bid.parse().unwrap(),
            )
        })
        .collect()
}
