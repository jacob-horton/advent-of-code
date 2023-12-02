fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        let (_, rest) = line.split_once(": ").expect("No colon found in line");
        let subsets = rest.split("; ");

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for subset in subsets {
            for colour_collection in subset.split(", ") {
                let (num, colour) = colour_collection
                    .split_once(" ")
                    .expect("No space found between number and colour");

                let num = num.parse::<u32>().expect("Not a number");
                match colour {
                    "red" => max_red = u32::max(max_red, num),
                    "green" => max_green = u32::max(max_green, num),
                    "blue" => max_blue = u32::max(max_blue, num),
                    _ => panic!("Unexpected colour"),
                };
            }
        }

        let power = max_red * max_green * max_blue;
        sum += power;
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

        assert_eq!(result, 2286);
    }
}
