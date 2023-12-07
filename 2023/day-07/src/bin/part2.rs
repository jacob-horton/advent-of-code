use day_07::parse_input;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut hand_bids = parse_input(input, true);
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
    use day_07::hand::Hand;
    use day_07::hand_type::HandType;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 5905);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 243101568);
    }

    #[test]
    fn test_hand_types() {
        let mappings = [
            ("32T1K", HandType::HighCard),
            ("32T3K", HandType::OnePair),
            ("QT32J", HandType::OnePair),
            ("KK677", HandType::TwoPair),
            ("AAA2Q", HandType::ThreeOfAKind),
            ("JJA12", HandType::ThreeOfAKind),
            ("QQAAJ", HandType::FullHouse),
            ("AAA22", HandType::FullHouse),
            ("T55J5", HandType::FourOfAKind),
            ("KTJJT", HandType::FourOfAKind),
            ("QQQAJ", HandType::FourOfAKind),
            ("JJJJJ", HandType::FiveOfAKind),
            ("JJJJQ", HandType::FiveOfAKind),
            ("QQQQJ", HandType::FiveOfAKind),
        ];

        for (cards, expected_type) in mappings {
            let hand = Hand::new(cards.chars().collect(), true);
            assert_eq!(hand.get_type(), expected_type);
        }
    }

    #[test]
    fn test_joker_comparison() {
        let mappings = [
            ("JJJJJ", "QQQQQ", std::cmp::Ordering::Less),
            ("JJJJJ", "JQQQQ", std::cmp::Ordering::Less),
            ("JAAAA", "22222", std::cmp::Ordering::Less),
            ("AAJJQ", "QAAJJ", std::cmp::Ordering::Greater),
            ("AAJJQ", "QAAAA", std::cmp::Ordering::Greater),
            ("AAAAJ", "22222", std::cmp::Ordering::Greater),
        ];

        for (cards1, cards2, expected_ordering) in mappings {
            let hand1 = Hand::new(cards1.chars().collect(), true);
            let hand2 = Hand::new(cards2.chars().collect(), true);
            assert_eq!(hand1.cmp(&hand2), expected_ordering);
        }
    }
}
