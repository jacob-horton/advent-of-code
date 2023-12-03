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

                for j in y.saturating_sub(1)..=usize::min(matrix.len() - 1, y + 1) {
                    for i in x.saturating_sub(1)..=usize::min(line.len() - 1, x + 1) {
                        if matrix[j][i] != '.' && !matrix[j][i].is_numeric() {
                            symbol_adjacent = true;
                        }
                    }
                }
            } else {
                if buffer.len() > 0 {
                    if symbol_adjacent {
                        sum += buffer.parse::<u32>().unwrap();
                    }

                    buffer.clear();
                    symbol_adjacent = false;
                }
            }
        }

        if buffer.len() > 0 {
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
}
