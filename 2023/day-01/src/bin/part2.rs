use std::env::args;

use day_01::{LinearSearchNumberFinder, NumberFinder, ReplaceNumberFinder};

fn main() {
    let finder: Box<dyn NumberFinder> = match args().nth(1).unwrap_or("linear".to_string()).as_str()
    {
        "linear" => Box::new(LinearSearchNumberFinder {}),
        "replace" => Box::new(ReplaceNumberFinder {}),
        _ => panic!("Invalid number finder. Can be 'linear' or 'replace'"),
    };

    let input = include_str!("../inputs/input.txt");
    let result = process(input, finder);
    println!("{result}");
}

fn process(input: &str, number_finder: Box<dyn NumberFinder>) -> u32 {
    let lines = input.split('\n');

    lines
        .map(|line| {
            if line.trim().is_empty() {
                return 0;
            }

            let first = number_finder.find_number(line, false);
            let last = number_finder.find_number(line, true);

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day_01::ReplaceNumberFinder;

    use super::*;

    #[test]
    fn test_input_linear() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input, Box::new(LinearSearchNumberFinder {}));

        assert_eq!(result, 281)
    }

    #[test]
    fn overlapping_word_numbers_linear() {
        let input = "afiveighttwo565twonea";
        let result = process(input, Box::new(LinearSearchNumberFinder {}));

        assert_eq!(result, 51)
    }

    #[test]
    fn two_of_same_digit_linear() {
        let input = "afivetwo565asdf6";
        let result = process(input, Box::new(LinearSearchNumberFinder {}));

        assert_eq!(result, 56)
    }

    #[test]
    fn two_of_same_word_number_linear() {
        let input = "a1two56two";
        let result = process(input, Box::new(LinearSearchNumberFinder {}));

        assert_eq!(result, 12)
    }

    #[test]
    fn real_input_linear() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input, Box::new(LinearSearchNumberFinder {}));

        assert_eq!(result, 54980)
    }

    #[test]
    fn test_input_replace() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input, Box::new(ReplaceNumberFinder {}));

        assert_eq!(result, 281)
    }

    #[test]
    fn overlapping_word_numbers_replace() {
        let input = "afiveighttwo565twonea";
        let result = process(input, Box::new(ReplaceNumberFinder {}));

        assert_eq!(result, 51)
    }

    #[test]
    fn two_of_same_digit_replace() {
        let input = "afivetwo565asdf6";
        let result = process(input, Box::new(ReplaceNumberFinder {}));

        assert_eq!(result, 56)
    }

    #[test]
    fn two_of_same_word_number_replace() {
        let input = "a1two56two";
        let result = process(input, Box::new(ReplaceNumberFinder {}));

        assert_eq!(result, 12)
    }

    #[test]
    fn real_input_replace() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input, Box::new(ReplaceNumberFinder {}));

        assert_eq!(result, 54980)
    }
}
