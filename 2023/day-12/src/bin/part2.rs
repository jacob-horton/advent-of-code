use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn bool_to_int(val: bool) -> u64 {
    if val {
        return 1;
    } else {
        return 0;
    }
}

fn get_possible_arrangements(
    line: &str,
    groups: &[u64],
    cache: &mut HashMap<(String, Vec<u64>), u64>,
) -> u64 {
    if let Some(c) = cache.get(&(line.to_owned(), groups.to_owned())) {
        return *c;
    }

    if line.len() == 0 {
        return bool_to_int(groups.len() == 0);
    }

    if groups.len() == 0 {
        return bool_to_int(!line.contains('#'));
    }

    match line.chars().next().unwrap() {
        '.' => get_possible_arrangements(&line[1..], groups, cache),
        '#' => {
            // If any in next group aren't a #, or the one after group is a #, not possible
            if line.chars().take(groups[0] as usize).any(|c| c == '.')
                || line.chars().nth(groups[0] as usize).unwrap_or('.') == '#'
            {
                0
            } else {
                // If line length matches next group and that's the last group, it's correct
                if line.len() == groups[0] as usize {
                    return bool_to_int(groups.len() == 1);
                }

                // If the line length is less than the size of the next group, it's not a correct
                // arrangement
                if line.len() < groups[0] as usize {
                    return 0;
                }

                let result =
                    get_possible_arrangements(&line[groups[0] as usize + 1..], &groups[1..], cache);

                cache.insert((line.to_owned(), groups.to_owned()), result);
                result
            }
        }
        '?' => {
            let result = get_possible_arrangements(&format!("#{}", &line[1..]), groups, cache)
                + get_possible_arrangements(&format!(".{}", &line[1..]), groups, cache);

            cache.insert((line.to_owned(), groups.to_owned()), result);
            result
        }
        _ => panic!("Unknown char"),
    }
}

fn process(input: &str) -> u64 {
    let mut sum = 0u64;
    for line in input.trim().split('\n') {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups: Vec<u64> = groups.split(',').map(|g| g.parse().unwrap()).collect();

        // Unfold
        let groups = groups.repeat(5);
        let mut new_springs = springs.to_string();
        for _ in 0..4 {
            new_springs += &format!("?{springs}");
        }

        let arr = get_possible_arrangements(&new_springs, &groups, &mut HashMap::new());
        sum += arr;
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 525152);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 4964259839627);
    }
}
