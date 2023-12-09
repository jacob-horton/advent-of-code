use day_05::{parse_input, Mapping};
use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn seed_exists(seeds: &[u64], seed: u64) -> bool {
    for parts in seeds.chunks(2) {
        let start = parts[0];
        let len = parts[1];

        if seed >= start && seed < start + len {
            return true;
        }
    }

    false
}

// Get start of each range mapped to location numbers
fn get_range_start_points(mappings: &Vec<Mapping>) -> Vec<u64> {
    let mut start_points = Vec::new();

    // Loop through each mapping, collecting the start of each range
    for mapping in mappings {
        // Add source start to starting points
        start_points.append(
            &mut mapping
                .mappings
                .iter()
                .map(|m| m.source_start)
                .collect_vec(),
        );

        // Map starting points to next category
        start_points = start_points
            .into_iter()
            .map(|i| mapping.map_value(i))
            .collect_vec();
    }

    // Order them and get unique values
    start_points.sort();
    start_points.into_iter().unique().collect_vec()
}

fn process(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);

    // Get starting points of each range of each mapping (mapped to location).
    // We know that the lowest location will be at the start of one of the
    // ranges, as within the ranges, it will only increase as you increase the
    // input
    let range_start_points = get_range_start_points(&mappings);

    // Work back from location to seed to find the first location point that
    // has a corresponding seed. As it is sorted, the first one we come across
    // will be the smallest
    for i in range_start_points {
        let mut v = i;

        for mapping in mappings.iter().rev() {
            v = mapping.reverse_map_value(v);
        }

        if seed_exists(&seeds, v) {
            return i;
        }
    }

    panic!("Not found");
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
