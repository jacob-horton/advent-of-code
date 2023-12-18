use std::collections::HashSet;

use day_18::parse_input_part1;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// TODO: optimise - maybe store as line segments oriented same way, or store all points
fn on_perimeter(perimeter: &[(i64, i64)], point: (i64, i64)) -> bool {
    for pairs in perimeter.windows(2) {
        let (a, b) = (pairs[0], pairs[1]);
        let min_x = i64::min(a.0, b.0);
        let max_x = i64::max(a.0, b.0);
        let min_y = i64::min(a.1, b.1);
        let max_y = i64::max(a.1, b.1);

        if min_x == max_x && min_x == point.0 && point.1 >= min_y && point.1 <= max_y {
            return true;
        }

        if min_y == max_y && min_y == point.1 && point.0 >= min_x && point.0 <= max_x {
            return true;
        }
    }

    false
}

fn get_point_inside(perimeter: &[(i64, i64)]) -> (i64, i64) {
    let start = perimeter
        .iter()
        .min_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .unwrap();

    // Try above and below leftmost corner to find point inside
    for dy in [-1, 1] {
        let point_on_perimeter = (start.0, start.1 + dy);
        let point_inside = (start.0 + 1, start.1 + dy);

        if on_perimeter(perimeter, point_on_perimeter) && !on_perimeter(perimeter, point_inside) {
            return point_inside;
        }
    }

    panic!("Could not find point inside");
}

fn flood_fill(perimeter: &[(i64, i64)], start: (i64, i64)) -> u64 {
    let mut frontier = vec![start];
    let mut explored = HashSet::new();

    while let Some(current) = frontier.pop() {
        if explored.contains(&current) {
            continue;
        }

        if on_perimeter(perimeter, current) {
            continue;
        }

        explored.insert(current);

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbour = (current.0 + dir.0, current.1 + dir.1);
            frontier.push(neighbour);
        }
    }

    explored.len() as u64
}

fn perimeter_len(perimeter: &[(i64, i64)]) -> u64 {
    let mut len = 0;
    for pairs in perimeter.windows(2) {
        let (a, b) = (pairs[0], pairs[1]);
        len += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    }

    len
}

fn process(input: &str) -> u64 {
    let instructions = parse_input_part1(input);
    let mut current = (0, 0);
    let mut perimeter = vec![current];

    for instruction in instructions {
        current = (
            current.0 + instruction.dir.0 * instruction.distance,
            current.1 + instruction.dir.1 * instruction.distance,
        );
        perimeter.push(current);
    }

    let point_inside = get_point_inside(&perimeter);
    let flood_filled = flood_fill(&perimeter, point_inside);
    let perimeter_len = perimeter_len(&perimeter);

    flood_filled + perimeter_len
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 62);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 49061);
    }
}
