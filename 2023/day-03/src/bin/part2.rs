fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn get_number(matrix: &[Vec<char>], pos: &(usize, usize)) -> u32 {
    let mut start = pos.0;
    while start > 0 && matrix[pos.1][start - 1].is_numeric() {
        start -= 1;
    }

    let mut end = pos.0;
    while end < matrix[pos.1].len() - 1 && matrix[pos.1][end + 1].is_numeric() {
        end += 1;
    }

    let number_string = String::from_iter(&matrix[pos.1][start..=end]);
    number_string.parse().unwrap()
}

fn get_unique_adjacent_number_positions(
    matrix: &Vec<Vec<char>>,
    symbol_pos: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut unique_adjacent_number_positions = Vec::new();

    let min_x = symbol_pos.0.saturating_sub(1);
    let min_y = symbol_pos.1.saturating_sub(1);

    let max_x = usize::min(matrix[0].len() - 1, symbol_pos.0 + 1);
    let max_y = usize::min(matrix.len() - 1, symbol_pos.1 + 1);

    for (j, row) in matrix.iter().enumerate().take(max_y + 1).skip(min_y) {
        let mut prev_numeric = false;
        for (i, val) in row.iter().enumerate().take(max_x + 1).skip(min_x) {
            if val.is_numeric() {
                if !prev_numeric {
                    unique_adjacent_number_positions.push((i, j));
                }
                prev_numeric = true;
            } else {
                prev_numeric = false;
            }
        }
    }

    unique_adjacent_number_positions
}

fn process(input: &str) -> u32 {
    let lines = input.split('\n').filter(|line| !line.trim().is_empty());
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'*' {
                let unique_adjacent_number_positions =
                    get_unique_adjacent_number_positions(&matrix, &(x, y));

                if unique_adjacent_number_positions.len() != 2 {
                    continue;
                }

                let numbers = unique_adjacent_number_positions
                    .iter()
                    .map(|pos| get_number(&matrix, pos));

                sum += numbers.product::<u32>();
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
        assert_eq!(result, 467835);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 84900879);
    }
}
