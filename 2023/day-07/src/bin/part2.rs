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
    fn test_joker() {
        let hand = Hand::new("32T3K".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::OnePair);

        let hand = Hand::new("T55J5".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand::new("KK677".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::TwoPair);

        let hand = Hand::new("KTJJT".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand::new("QQQJA".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand::new("JJJJJ".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand::new("JJJJQ".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand::new("QQQQJ".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FiveOfAKind);

        let hand = Hand::new("QQQAJ".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FourOfAKind);

        let hand = Hand::new("QQAAJ".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand::new("QT32J".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::OnePair);

        let hand = Hand::new("AAA2Q".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::ThreeOfAKind);

        let hand = Hand::new("AAA22".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand::new("JAA22".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::FullHouse);

        let hand = Hand::new("JJA12".chars().collect(), true);
        assert_eq!(hand.get_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_joker_comparison() {
        let hand = Hand::new("JJJJJ".chars().collect(), true);
        let other = Hand::new("QQQQQ".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);

        let hand = Hand::new("JJJJJ".chars().collect(), true);
        let other = Hand::new("JQQQQ".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);

        let hand = Hand::new("AAJJQ".chars().collect(), true);
        let other = Hand::new("QAAJJ".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand::new("AAJJQ".chars().collect(), true);
        let other = Hand::new("QAAAA".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand::new("AAAAJ".chars().collect(), true);
        let other = Hand::new("22222".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Greater);

        let hand = Hand::new("JAAAA".chars().collect(), true);
        let other = Hand::new("22222".chars().collect(), true);
        assert_eq!(hand.cmp(&other), std::cmp::Ordering::Less);
    }
}
