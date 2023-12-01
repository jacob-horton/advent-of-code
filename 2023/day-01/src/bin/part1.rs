fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let lines = input.split('\n');

    lines
        .map(|line| {
            if line.trim().is_empty() {
                return 0;
            }

            let first = line.chars().find(|c: &char| c.is_numeric()).unwrap();
            let last = line.chars().rev().find(|c: &char| c.is_numeric()).unwrap();

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);

        assert_eq!(result, 142)
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);

        assert_eq!(result, 55816)
    }
}
