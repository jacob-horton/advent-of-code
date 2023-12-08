use std::collections::HashMap;

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    let mut nodes = HashMap::new();
    let (instructions, nodes_raw) = input.split_once("\n\n").unwrap();

    for node in nodes_raw.split('\n') {
        if node.trim().is_empty() {
            continue;
        }

        let (node_name, rest) = node.split_once(" = ").unwrap();
        let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();

        nodes.insert(node_name, Node { left, right });
    }

    let mut steps = 0;
    let mut curr_name = "AAA";
    for instruction in instructions.chars().into_iter().cycle() {
        if curr_name == "ZZZ" {
            break;
        }

        let curr = nodes.get(curr_name).unwrap();
        match instruction {
            'L' => curr_name = curr.left,
            'R' => curr_name = curr.right,
            _ => panic!("Unknown instruction"),
        }

        steps += 1;
    }

    steps
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../inputs/test2_part1.txt");
        let result = process(input);
        assert_eq!(result, 6);
    }
}
