use day_17::{find_shortest_path, parse_input};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let grid = parse_input(input);
    find_shortest_path(
        &grid,
        (0, 0),
        (grid[0].len() as i32 - 1, grid.len() as i32 - 1),
        4,
        10,
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 94);
    }

    #[test]
    fn test_input_2() {
        let input = include_str!("../inputs/test_part2_2.txt");
        let result = process(input);
        assert_eq!(result, 71);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 993);
    }
}
