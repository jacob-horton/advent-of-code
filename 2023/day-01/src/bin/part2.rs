static WORD_NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn find_number(line: &str, from_end: bool) -> u32 {
    let mut char_indices: Vec<(usize, char)> = line.char_indices().collect();
    if from_end {
        char_indices = char_indices.into_iter().rev().collect();
    }

    for (i, char) in char_indices {
        // Check if there is a spelled out number
        let word_number = WORD_NUMBERS.iter().find(|w| {
            if from_end {
                line[i..].starts_with(*w)
            } else {
                line[..i].ends_with(*w)
            }
        });
        if let Some(word_number) = word_number {
            let digit = WORD_NUMBERS.iter().position(|w| w == word_number).unwrap();
            return digit as u32 + 1; // Add 1 because 0 indexed
        }

        // Check for digit
        if char.is_numeric() {
            return char.to_digit(10).unwrap();
        }
    }

    panic!("Shouldn't happen. Invalid input - no numbers found")
}

fn process(input: &str) -> u32 {
    let lines = input.split('\n');

    lines
        .map(|line| {
            if line.trim().is_empty() {
                return 0;
            }

            let first = find_number(line, false);
            let last = find_number(line, true);

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);

        assert_eq!(result, 281)
    }

    #[test]
    fn overlapping_word_numbers() {
        let input = "afiveighttwo565twonea";
        let result = process(input);

        assert_eq!(result, 51)
    }

    #[test]
    fn two_of_same_digit() {
        let input = "afivetwo565asdf6";
        let result = process(input);

        assert_eq!(result, 56)
    }

    #[test]
    fn two_of_same_word_number() {
        let input = "a1two56two";
        let result = process(input);

        assert_eq!(result, 12)
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);

        assert_eq!(result, 54980)
    }
}
