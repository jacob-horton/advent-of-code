fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn distance_travelled(total_time: u64, time_holding: u64) -> u64 {
    (total_time - time_holding) * time_holding
}

fn parse_line(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .trim()
        .parse()
        .unwrap()
}

// Binary search to find the first hold_time that goes further than `distance`
fn find_first_further(time: u64, distance: u64) -> u64 {
    // Somewhere between start and centre, so start in the middle of those
    let mut start = 0;
    let mut end = time / 2;
    let mut curr = end / 2;

    while curr != start && curr != end {
        if distance_travelled(time, curr) > distance {
            end = curr;
        } else {
            start = curr;
        }

        curr = (end - start) / 2 + start;
    }

    curr + 1
}

fn process(input: &str) -> u64 {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let time = parse_line(time_line);
    let distance = parse_line(distance_line);

    // #  = Slower
    // @  = Faster
    // \d = Time holding
    //
    // 012345678
    // ##@@@@@##
    //
    // Symmetrical, so if we know the first point at which the time is faster,
    // we know the last point at which it is faster (time - first_faster). Then
    // we can do a subtraction to find the number of different times inbetween
    let first_further = find_first_further(time, distance);
    (time - first_further) - first_further + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 42250895);
    }
}
