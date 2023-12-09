use day_08::lcm;
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

fn process(input: &str) -> u64 {
    let mut nodes = HashMap::new();
    let (instructions, nodes_raw) = input.split_once("\n\n").unwrap();

    let mut starting_nodes = Vec::new();
    for node in nodes_raw.split('\n') {
        if node.trim().is_empty() {
            continue;
        }

        let (node_name, rest) = node.split_once(" = ").unwrap();
        let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();

        if node_name.ends_with('A') {
            starting_nodes.push(node_name);
        }

        nodes.insert(node_name, Node { left, right });
    }

    let mut first_z_steps = vec![None; starting_nodes.len()];

    // Note: this assumes that the Z found is at the end of the instructions
    // This means that it will have a period that is the number of steps until finding the first Z
    // This is just an observation from the test and real data
    let mut steps = 0;
    let mut curr_node_names = starting_nodes;
    for instruction in instructions.chars().cycle() {
        if !first_z_steps.contains(&None) {
            break;
        }

        // Loop through each node
        for (i, curr_name) in curr_node_names.iter_mut().enumerate() {
            if first_z_steps[i].is_some() {
                continue;
            }

            if curr_name.ends_with('Z') {
                first_z_steps[i] = Some(steps);
                continue;
            }

            let curr_node = nodes.get(curr_name).unwrap();
            match instruction {
                'L' => *curr_name = curr_node.left,
                'R' => *curr_name = curr_node.right,
                _ => panic!("Unknown instruction"),
            }
        }

        steps += 1;
    }

    lcm(&first_z_steps.into_iter().flatten().collect::<Vec<u64>>())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 9858474970153);
    }
}
