fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn find_derivative(sequence: &[i32]) -> Vec<i32> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn next_number(sequence: &[i32]) -> i32 {
    let mut derivatives: Vec<Vec<i32>> = vec![sequence.to_owned()];
    while derivatives.last().unwrap().iter().any(|x| x != &0) {
        derivatives.push(find_derivative(derivatives.last().unwrap()));
    }

    derivatives.into_iter().map(|d| *d.last().unwrap()).sum()
}

fn process(input: &str) -> i32 {
    let sequences: Vec<_> = input
        .split('\n')
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            x.trim()
                .split(' ')
                .map(|y| y.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    sequences.into_iter().map(|s| next_number(&s)).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 1681758908);
    }
}
