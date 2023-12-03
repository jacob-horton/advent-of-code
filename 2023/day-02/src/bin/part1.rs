use day_02::parse_input;

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let games = parse_input(input);
    games
        .into_iter()
        .filter(|game| {
            game.subsets.iter().all(|subset| {
                subset.red <= MAX_RED && subset.green <= MAX_GREEN && subset.blue <= MAX_BLUE
            })
        })
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);

        assert_eq!(result, 8);
    }

    #[test]
    pub fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);

        assert_eq!(result, 2512);
    }
}
