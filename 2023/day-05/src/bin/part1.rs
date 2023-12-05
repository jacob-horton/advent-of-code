use day_05::parse_input_from_key;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u64 {
    let (seeds, mappings) = parse_input_from_key(input);
    let mut from = "seed";

    let mut values = seeds;
    while from != "location" {
        let mapping = mappings.get(from).unwrap();

        values = values.iter().map(|v| mapping.map_value(*v)).collect();
        from = mapping.to;
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
