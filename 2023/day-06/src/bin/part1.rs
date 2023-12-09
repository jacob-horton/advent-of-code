fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn distance_travelled(total_time: u32, time_holding: u32) -> u32 {
    (total_time - time_holding) * time_holding
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn process(input: &str) -> u32 {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let times = parse_line(time_line);
    let distances = parse_line(distance_line);

    times
        .into_iter()
        .zip(distances)
        .map(|(t, max_d)| {
            (0..t)
                .filter(|hold_time| distance_travelled(t, *hold_time) > max_d)
                .count() as u32
        })
        .product()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 449820);
    }
}
