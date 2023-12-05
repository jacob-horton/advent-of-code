use day_05::parse_input;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);

    let mut values = seeds;
    for mapping in mappings {
        values = values.iter().map(|v| mapping.map_value(*v)).collect();
    }

    values.into_iter().min().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 35);
    }
}
