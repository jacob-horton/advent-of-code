fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;

    for card in input.split('\n') {
        if card.trim().is_empty() {
            continue;
        }

        let (_, numbers) = card.split_once(": ").unwrap();
        let (winning, mine) = numbers.split_once(" | ").unwrap();

        let mine: Vec<_> = mine.split(' ').filter(|n| !n.trim().is_empty()).collect();
        let mine_winning: Vec<_> = winning.split(' ').filter(|n| mine.contains(n)).collect();

        if !mine_winning.is_empty() {
            sum += 2u32.pow(mine_winning.len() as u32 - 1);
        }
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 15268);
    }
}
