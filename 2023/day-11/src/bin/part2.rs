fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input, 1_000_000);
    println!("{result}");
}

fn process(input: &str, expand_size: u64) -> u64 {
    let expand_size = expand_size - 1; // Subtract one for existing empty row/column
    let mut galaxies = Vec::new();

    let mut max_y = 0;
    let mut max_x = 0;
    for (j, line) in input.trim().split('\n').enumerate() {
        max_y += 1;
        max_x = line.len();

        for (i, point) in line.chars().enumerate() {
            if point == '#' {
                galaxies.push((i as u64, j as u64))
            }
        }
    }

    // Just keep track of empty rows/cols
    let empty_rows: Vec<_> = (0..max_y as u64)
        .filter(|j| !galaxies.iter().any(|(_, y)| y == j))
        .collect();
    let empty_cols: Vec<_> = (0..max_x as u64)
        .filter(|i| !galaxies.iter().any(|(x, _)| x == i))
        .collect();

    let mut total_distance = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            let max_x = u64::max(g1.0, g2.0);
            let min_x = u64::min(g1.0, g2.0);
            let max_y = u64::max(g1.1, g2.1);
            let min_y = u64::min(g1.1, g2.1);

            let empty_row_count = empty_rows
                .iter()
                .filter(|r| *r > &min_y && *r <= &max_y)
                .count();
            let empty_col_count = empty_cols
                .iter()
                .filter(|c| *c > &min_x && *c <= &max_x)
                .count();

            let empty_count = empty_row_count + empty_col_count;
            let dist = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1) + empty_count as u64 * expand_size;
            total_distance += dist;
        }
    }

    total_distance
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_input_alt() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_input_alt2() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input, 2);
        assert_eq!(result, 10154062);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_input3() {
        let input = include_str!("../inputs/test_part2_2.txt");
        let result = process(input, 1_000_000);
        assert_eq!(result, 4_000_002 + 2_000_002 + 4_000_002);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input, 1_000_000);
        assert_eq!(result, 553083047914);
    }
}
