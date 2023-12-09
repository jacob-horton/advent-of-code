pub const HAND_TYPE_STRENGTHS: [HandType; 7] = [
    HandType::HighCard,
    HandType::OnePair,
    HandType::TwoPair,
    HandType::ThreeOfAKind,
    HandType::FullHouse,
    HandType::FourOfAKind,
    HandType::FiveOfAKind,
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_strength = HAND_TYPE_STRENGTHS.iter().position(|t| t == self).unwrap();
        let other_strength = HAND_TYPE_STRENGTHS.iter().position(|t| t == other).unwrap();

        this_strength.cmp(&other_strength)
    }
}
