use std::collections::HashMap;

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Loop {
    z_index: u64,
    length: u64,
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
    let mut loops = vec![None; starting_nodes.len()];

    // Note: this only works if whenever it reaches the same node, it will be on the same
    // instruction
    let mut steps = 0;
    let mut curr_node_names = starting_nodes;
    for instruction in instructions.chars().into_iter().cycle() {
        if !loops.contains(&None) {
            break;
        }

        // Loop through each node
        for (i, curr_name) in curr_node_names.iter_mut().enumerate() {
            if loops[i].is_some() {
                continue;
            }

            if curr_name.ends_with('Z') {
                match first_z_steps[i] {
                    Some(first_z_step) => {
                        loops[i] = Some(Loop {
                            z_index: first_z_step,
                            length: steps - first_z_step,
                        });
                    }
                    None => {
                        first_z_steps[i] = Some(steps);
                    }
                }
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

    find_common_multiple(&loops.into_iter().flatten().collect())
}

// TODO: optimise
fn find_common_multiple(loops: &Vec<Loop>) -> u64 {
    let mut positions: Vec<u64> = loops.iter().map(|l| l.z_index).collect();

    // Assuming there is at least one loop
    while !positions.iter().all(|p| p == &positions[0]) {
        let (smallest_idx, smallest) = positions
            .iter()
            .enumerate()
            .min_by(|(_, p1), (_, p2)| p1.cmp(p2))
            .unwrap();

        let largest = positions.iter().max_by(|p1, p2| p1.cmp(p2)).unwrap();

        let multiple =
            ((largest - smallest) as f64 / loops[smallest_idx].length as f64).ceil() as u64;
        positions[smallest_idx] += loops[smallest_idx].length * multiple;
    }

    positions[0]
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_multiple_2_loops() {
        let loop1 = Loop {
            z_index: 4,
            length: 5,
        };
        let loop2 = Loop {
            z_index: 3,
            length: 4,
        };

        // 4, 9, 14, 19
        // 3, 7, 11, 15, 19
        let result = find_common_multiple(&vec![loop1, loop2]);
        assert_eq!(result, 19);
    }

    #[test]
    fn test_multiple_3_loops() {
        let loop1 = Loop {
            z_index: 4,
            length: 5,
        };
        let loop2 = Loop {
            z_index: 3,
            length: 4,
        };
        let loop3 = Loop {
            z_index: 9,
            length: 1,
        };

        // 4, 9, 14, 19
        // 3, 7, 11, 15, 19
        // 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
        let result = find_common_multiple(&vec![loop1, loop2, loop3]);
        assert_eq!(result, 19);
    }

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
