fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn find_derivative(sequence: &Vec<i32>) -> Vec<i32> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn next_number(sequence: &Vec<i32>) -> i32 {
    let mut derivatives: Vec<Vec<i32>> = vec![sequence.to_owned()];
    while derivatives.last().unwrap().iter().any(|x| x != &0) {
        derivatives.push(find_derivative(derivatives.last().unwrap()));
    }

    derivatives
        .into_iter()
        .map(|d| d.last().unwrap().clone())
        .sum()
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
    fn test_sequence() {
        let input = "-1 8 27 58 122 282 681 1611 3653 7980 17022 35902 75440 158200 330196 682764 1392266 2790739 5490489 10604640 20143903".split(' ').map(|v| v.parse().unwrap()).collect();
        let result = next_number(&input);
        assert_eq!(result, 37748543)
    }

    #[test]
    fn test_sequence_2() {
        let input = "-2 4 12 17 14 -2 -36 -93 -178 -296 -452 -651 -898 -1198 -1556 -1977 -2466 -3028 -3668 -4391 -5202".split(' ').map(|v| v.parse().unwrap()).collect();
        let result = next_number(&input);
        assert_eq!(result, -6106)
    }

    #[test]
    fn test_sequence_3() {
        let input = "-2 -4 -6 -8"
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();
        let result = next_number(&input);
        assert_eq!(result, -10)
    }

    #[test]
    fn test_sequence_4() {
        let input = "-2 -4 -8 -14"
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();
        let result = next_number(&input);
        assert_eq!(result, -22)
    }
}
