use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    let mut multipliers: HashMap<u32, u32> = HashMap::new();

    for card in input.split('\n') {
        if card.trim().is_empty() {
            continue;
        }

        let (card, numbers) = card.split_once(": ").unwrap();
        let (winning, mine) = numbers.split_once(" | ").unwrap();

        let (_, card_no) = card.split_once(' ').unwrap();
        let card_no: u32 = card_no.trim().parse().unwrap();

        let mine: Vec<_> = mine.split(' ').filter(|n| !n.trim().is_empty()).collect();
        let mine_winning: Vec<_> = winning.split(' ').filter(|n| mine.contains(n)).collect();

        let this_multiplier = *multipliers.get(&card_no).unwrap_or(&1);
        sum += this_multiplier; // Add the number of copies of this card that we have

        if !mine_winning.is_empty() {
            // Add extra cards to the ones after
            for i in card_no + 1..card_no + 1 + mine_winning.len() as u32 {
                let entry = multipliers.entry(i).or_insert(1);
                *entry += this_multiplier;
            }
        }
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 6283755);
    }
}
