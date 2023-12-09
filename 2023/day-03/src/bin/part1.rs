fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let lines = input.split('\n').filter(|line| !line.trim().is_empty());
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        let mut buffer = String::new();
        let mut symbol_adjacent = false;

        for (x, char) in line.iter().enumerate() {
            if char.is_numeric() {
                buffer.push(*char);

                let min_x = x.saturating_sub(1);
                let min_y = y.saturating_sub(1);

                let max_x = usize::min(line.len() - 1, x + 1);
                let max_y = usize::min(matrix.len() - 1, y + 1);

                for row in matrix.iter().take(max_y + 1).skip(min_y) {
                    for val in row.iter().take(max_x + 1).skip(min_x) {
                        if val != &'.' && !val.is_numeric() {
                            symbol_adjacent = true;
                        }
                    }
                }
            } else if !buffer.is_empty() {
                if symbol_adjacent {
                    sum += buffer.parse::<u32>().unwrap();
                }

                buffer.clear();
                symbol_adjacent = false;
            }
        }

        if !buffer.is_empty() {
            if symbol_adjacent {
                sum += buffer.parse::<u32>().unwrap();
            }

            buffer.clear()
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 530849);
    }
}
