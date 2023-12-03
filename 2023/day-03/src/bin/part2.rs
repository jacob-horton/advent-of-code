fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn get_number(matrix: &Vec<Vec<char>>, pos: &(usize, usize)) -> u32 {
    let mut start = pos.0;
    while start > 0 && matrix[pos.1][start - 1].is_numeric() {
        start -= 1;
    }

    let mut end = pos.0;
    while end < matrix[pos.1].len() - 1 && matrix[pos.1][end + 1].is_numeric() {
        end += 1;
    }

    let number_str = &matrix[pos.1][start..=end];
    let number_string: String = number_str.to_owned().into_iter().collect();
    number_string.parse().unwrap()
}

fn process(input: &str) -> u32 {
    let lines = input.split('\n').filter(|line| !line.trim().is_empty());
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'*' {
                let mut unique_adjacent_number_positions = Vec::new();

                for j in y.saturating_sub(1)..=usize::min(matrix.len() - 1, y + 1) {
                    let mut prev_numeric = false;
                    for i in x.saturating_sub(1)..=usize::min(line.len() - 1, x + 1) {
                        if matrix[j][i].is_numeric() {
                            if !prev_numeric {
                                unique_adjacent_number_positions.push((i, j));
                            }
                            prev_numeric = true;
                        } else {
                            prev_numeric = false;
                        }
                    }
                }

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
}
