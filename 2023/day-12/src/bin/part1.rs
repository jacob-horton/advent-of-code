use day_12::Spring;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn get_contiguous_damaged(springs: &Vec<Spring>) -> Vec<u32> {
    let mut contiguous_damaged = Vec::new();

    let mut last_non_damaged: i32 = -1;
    for (i, s) in springs.iter().enumerate() {
        match s {
            Spring::Damaged => {}
            _ => {
                let diff = i as i32 - last_non_damaged - 1;
                if diff > 0 {
                    contiguous_damaged.push(diff as u32);
                }

                last_non_damaged = i as i32;
            }
        }
    }
    let diff = springs.len() as i32 - last_non_damaged - 1;
    if diff > 0 {
        contiguous_damaged.push(diff as u32);
    }

    contiguous_damaged
}

fn fill_next_unknown(partial_springs: &Vec<Spring>, val: Spring) -> Vec<Spring> {
    let pos = partial_springs
        .iter()
        .position(|s| s == &Spring::Unknown)
        .unwrap();

    let mut new_springs = partial_springs.clone();
    new_springs[pos] = val;

    new_springs
}

fn line_arrangements(partial_springs: &Vec<Spring>, contiguous_damaged: &Vec<u32>) -> u32 {
    if !partial_springs.contains(&Spring::Unknown) {
        let this_contiguous_damaged = get_contiguous_damaged(partial_springs);
        if &this_contiguous_damaged == contiguous_damaged {
            return 1;
        } else {
            return 0;
        }
    }

    let fill_operational = fill_next_unknown(partial_springs, Spring::Operational);
    let fill_damaged = fill_next_unknown(partial_springs, Spring::Damaged);

    line_arrangements(&fill_operational, contiguous_damaged)
        + line_arrangements(&fill_damaged, contiguous_damaged)
}

fn process(input: &str) -> u32 {
    let mut arrangements = 0;
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        let (partial_springs_str, contiguous_damaged_str) = line.split_once(' ').unwrap();
        let partial_springs = partial_springs_str
            .chars()
            .map(|c| match c {
                '?' => Spring::Unknown,
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                _ => panic!("Unknown spring type"),
            })
            .collect();

        let contiguous_damaged = contiguous_damaged_str
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let a = line_arrangements(&partial_springs, &contiguous_damaged);
        arrangements += a;
    }

    arrangements
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 21);
    }

    // #[test]
    // fn real_input() {
    //     let input = include_str!("../inputs/input.txt");
    //     let result = process(input);
    //     assert_eq!(result, 7622);
    // }
}
