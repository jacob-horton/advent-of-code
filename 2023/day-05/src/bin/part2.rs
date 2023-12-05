use day_05::{parse_input, Mapping};
use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
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

// Get boundaries from each range in the location "realm"
fn get_intersection_points(mappings: &Vec<Mapping>) -> Vec<u64> {
    let mut intersection_points = Vec::new();

    // Loop through each mapping, collecting the end points of each range
    for mapping in mappings {
        // Add source end points to intersection points
        intersection_points.append(
            &mut mapping
                .mappings
                .iter()
                .map(|m| [m.source_start, m.source_start + m.range_length])
                .flatten()
                .collect_vec(),
        );

        // Map intersections to next "realm"
        intersection_points = intersection_points
            .into_iter()
            .map(|i| mapping.map_value(i))
            .collect_vec();
    }

    // Order them and get unique values
    intersection_points.sort();
    intersection_points.into_iter().unique().collect_vec()
}

fn process(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);

    // Get the intersection points which define the boundaries of mapping ranges.
    // We know that the lowest location will be at the start of one of these boundaries
    // as within the boundaries, it will only increase as you increase the input
    let intersection_points = get_intersection_points(&mappings);

    // Work back from location to seed to find the first location point
    // that has a corresponding seed
    for i in intersection_points {
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
