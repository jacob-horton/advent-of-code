use day_05::{parse_input_to_key, Mapping, MappingLine};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// Mapping = from humidity, to location
fn get_sorted_location_ranges(locations: &Mapping) -> Vec<MappingLine> {
    let mut sorted_mappings = locations.mappings.clone();
    sorted_mappings.sort_by(|m1, m2| m1.dest_start.cmp(&m2.dest_start));

    sorted_mappings
}

fn seed_exists(seeds: &Vec<u64>, seed: u64) -> bool {
    for parts in seeds.chunks(2) {
        let start = parts[0];
        let len = parts[1];

        if seed >= start && seed < start + len {
            return true;
        }
    }

    false
}

fn process(input: &str) -> u64 {
    let (seeds, mappings) = parse_input_to_key(input);

    let mut i = 0;
    loop {
        let mut v = i;
        let mut to = "location";

        while to != "seed" {
            let mapping = mappings.get(to).unwrap();
            v = mapping.reverse_map_value(v);
            to = mapping.from;
        }

        if seed_exists(&seeds, v) {
            return i;
        }

        i += 1;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 100165128);
    }
}
