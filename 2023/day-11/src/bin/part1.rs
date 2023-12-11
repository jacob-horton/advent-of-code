fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut galaxies = Vec::new();

    for (j, line) in input.split('\n').enumerate() {
        for (i, point) in line.chars().enumerate() {
            if point == '#' {
                galaxies.push((i, j))
            }
        }
    }

    let max_x = galaxies
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .unwrap()
        .0;
    let max_y = galaxies
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
        .1;

    // Galaxy positions after "gravitational effects"
    let mut new_galaxies = galaxies.clone();

    let mut expanded = 0;
    for j in 0..max_y {
        if !galaxies.iter().any(|(_, y)| y == &j) {
            new_galaxies
                .iter_mut()
                .filter(|(_, y)| *y > j + expanded)
                .for_each(|g| g.1 += 1);

            expanded += 1;
        }
    }

    let mut expanded = 0;
    for i in 0..max_x {
        if !galaxies.iter().any(|(x, _)| x == &i) {
            new_galaxies
                .iter_mut()
                .filter(|(x, _)| *x > i + expanded)
                .for_each(|g| g.0 += 1);

            expanded += 1;
        }
    }

    let mut total_distance = 0;
    for g1 in &new_galaxies {
        for g2 in &new_galaxies {
            if g1 == g2 {
                continue;
            }

            let dist = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            total_distance += dist;
        }
    }

    // Divide by 2 because we're counting each pair twice
    total_distance as u32 / 2
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 10154062);
    }
}
