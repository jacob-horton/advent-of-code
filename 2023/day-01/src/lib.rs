static WORD_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static OVERLAPS: [(&str, &str); 7] = [
    ("oneight", "oneeight"),
    ("twone", "twoone"),
    ("threeight", "threeeight"),
    ("fiveight", "fiveeight"),
    ("sevenine", "sevennine"),
    ("eightwo", "eighttwo"),
    ("nineight", "nineeight"),
];

pub trait NumberFinder {
    fn find_number(&self, line: &str, from_end: bool) -> u32;
}

// Search substring from start/end until it contains a word number or digit
// Harder to follow, but more efficient
pub struct LinearSearchNumberFinder {}
impl NumberFinder for LinearSearchNumberFinder {
    fn find_number(&self, line: &str, from_end: bool) -> u32 {
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
}

// Replace word numbers with digits, then find digits
// Easier to follow, but less efficient
pub struct ReplaceNumberFinder {}
impl NumberFinder for ReplaceNumberFinder {
    fn find_number(&self, line: &str, from_end: bool) -> u32 {
        let mut replaced_line = line.to_string();

        // Replace any overlapping words with separate ones
        for (k, v) in OVERLAPS {
            replaced_line = replaced_line.replace(k, v);
        }

        // Replace any words with digits
        for (i, w) in WORD_NUMBERS.iter().enumerate() {
            replaced_line = replaced_line.replace(w, &(i + 1).to_string());
        }

        // Find digit
        let char = if from_end {
            replaced_line.chars().rfind(|c: &char| c.is_numeric())
        } else {
            replaced_line.chars().find(|c: &char| c.is_numeric())
        };

        char.unwrap().to_digit(10).unwrap()
    }
}
