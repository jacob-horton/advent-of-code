fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// TODO: tidy
fn process(input: &str) -> u32 {
    let lines = input.split('\n');
    let word_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    lines
        .map(|line| {
            if line.trim().is_empty() {
                return 0;
            }

            let first_numeric_i = line.chars().position(|c: char| c.is_numeric());
            let last_numeric_i = line
                .chars()
                .rev()
                .position(|c: char| c.is_numeric())
                .map(|i| line.len() - i - 1);

            let first_word = word_numbers
                .iter()
                .filter_map(|word_number| Some((word_number, line.find(word_number)?)))
                .min_by(|(_, i1), (_, i2)| i1.cmp(i2));

            let last_word = word_numbers
                .iter()
                .filter_map(|word_number| Some((word_number, line.rfind(word_number)?)))
                .max_by(|(_, i1), (_, i2)| i1.cmp(i2));

            let first = match first_word {
                Some((w, i)) => {
                    let mut val = None;
                    if let Some(first_numeric_i) = first_numeric_i {
                        if i > first_numeric_i {
                            val = line.chars().nth(first_numeric_i).unwrap().to_digit(10);
                        }
                    }

                    val.unwrap_or(word_numbers.iter().position(|x| x == w).unwrap() as u32 + 1)
                }
                None => line
                    .chars()
                    .nth(first_numeric_i.unwrap())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            };

            let last = match last_word {
                Some((w, i)) => {
                    let mut val = None;
                    if let Some(last_numeric_i) = last_numeric_i {
                        if i < last_numeric_i {
                            val = line.chars().nth(last_numeric_i).unwrap().to_digit(10);
                        }
                    }

                    val.unwrap_or(word_numbers.iter().position(|x| x == w).unwrap() as u32 + 1)
                }
                None => line
                    .chars()
                    .nth(last_numeric_i.unwrap())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            };

            format!("{first}{last}").parse::<u32>().unwrap()
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
