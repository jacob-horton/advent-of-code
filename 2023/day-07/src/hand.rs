use crate::{get_counts, hand_type::HandType};

const CARD_STRENGTHS_BASIC: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_STRENGTHS_JOKER_WILDCARD: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

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
                // If type of hand matches, check individual cards
                for (c1, c2) in self.cards.iter().zip(&other.cards) {
                    let c1_strength = card_strengths.iter().position(|c| c == c1).unwrap();
                    let c2_strength = card_strengths.iter().position(|c| c == c2).unwrap();

                    match c1_strength.cmp(&c2_strength) {
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
