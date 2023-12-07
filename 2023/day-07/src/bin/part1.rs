use day_07::parse_input;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut hand_bids = parse_input(input, false);
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
