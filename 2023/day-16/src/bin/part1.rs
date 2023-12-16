use day_16::{get_energised_tiles, parse_input, Beam};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let grid = parse_input(input);

    get_energised_tiles(
        &grid,
        Beam {
            pos: (0, 0),
            dir: (1, 0),
        },
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 8021);
    }
}
