use day_10::{get_next, parse_input};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let (start, grid) = parse_input(input);

    let mut curr_1 = start;
    let mut curr_2 = start;

    // NOTE: doesn't work if start.x == 0
    let mut next_1 = get_next((start.0 + 1, start.1), start, &grid);
    let mut next_2 = get_next(next_1, start, &grid);

    let mut count = 0;
    while next_1 != next_2 {
        let new_next_1 = get_next(curr_1, next_1, &grid);
        let new_next_2 = get_next(curr_2, next_2, &grid);

        curr_1 = next_1;
        curr_2 = next_2;

        next_1 = new_next_1;
        next_2 = new_next_2;

        count += 1;
    }

    count + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../inputs/test_part1_2.txt");
        let result = process(input);
        assert_eq!(result, 8);
    }
}
