pub mod hand;
pub mod hand_type;

use crate::hand::Hand;
use std::collections::HashMap;

// Get the count of each kind of card
pub fn get_counts(cards: &Vec<char>) -> Vec<(char, u32)> {
    let mut counts = HashMap::new();

    for card in cards {
        let entry = counts.entry(*card).or_insert(0);
        *entry += 1;
    }

    counts.into_iter().collect()
}

// Returns Vec<(hand, bid)>
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
