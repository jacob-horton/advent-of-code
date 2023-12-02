static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;

    'outer: for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        let (game, rest) = line.split_once(": ").expect("No colon found in line");
        let (_, id) = game
            .split_once(" ")
            .expect("No space found between 'Game' and ID");
        let subsets = rest.split("; ");

        for subset in subsets {
            for colour_collection in subset.split(", ") {
                let (num, colour) = colour_collection
                    .split_once(" ")
                    .expect("No space found between number and colour");

                let max = match colour {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    "blue" => MAX_BLUE,
                    _ => panic!("Unexpected colour"),
                };

                if num.parse::<u32>().expect("Not a number") > max {
                    continue 'outer;
                }
            }
        }

        sum += id.parse::<u32>().expect("ID not a number");
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);

        assert_eq!(result, 8);
    }
}
