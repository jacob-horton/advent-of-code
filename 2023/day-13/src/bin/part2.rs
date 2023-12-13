use day_13::{does_reflect, get_bit_patterns};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let patterns = get_bit_patterns(input);

    let mut horiz_mirrors = 0u32;
    let mut vert_mirrors = 0u32;
    for pattern in &patterns {
        // NOTE: this does not check the smudges are in the same position
        for i in 1..pattern.vertical.len() {
            if does_reflect(&pattern.vertical, i as u32, 1, true) {
                vert_mirrors += i as u32;
                break;
            }
        }

        for i in 1..pattern.horizontal.len() {
            if does_reflect(&pattern.horizontal, i as u32, 1, true) {
                horiz_mirrors += i as u32;
                break;
            }
        }
    }

    vert_mirrors + 100 * horiz_mirrors
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 400);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 32497);
    }
}
