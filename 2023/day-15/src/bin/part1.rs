fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn hash(val: &str) -> u32 {
    let mut hash = 0;
    for ascii in val.bytes() {
        hash += ascii as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn process(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 1320);
    }
}
