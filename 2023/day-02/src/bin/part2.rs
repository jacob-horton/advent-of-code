use day_02::{parse_input, Subset};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .into_iter()
        .map(|game| {
            let max = game
                .subsets
                .into_iter()
                .reduce(|acc, s| Subset::piecewise_max(acc, s))
                .unwrap();

            max.red * max.green * max.blue
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);

        assert_eq!(result, 2286);
    }

    #[test]
    pub fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);

        assert_eq!(result, 67335);
    }
}
