fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    // y position (from top) of the last rock in each column
    let mut obstacles = vec![0; input.split('\n').next().unwrap().len()];
    let height = input.trim().split('\n').count();

    let mut sum = 0;
    for (j, line) in input.trim().split('\n').enumerate() {
        for (i, char) in line.char_indices() {
            match char {
                '#' => obstacles[i] = j + 1,
                'O' => {
                    sum += height - obstacles[i];
                    obstacles[i] += 1
                }
                _ => (),
            }
        }
    }

    sum as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 136);
    }
}
