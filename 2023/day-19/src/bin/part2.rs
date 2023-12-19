use std::collections::HashMap;

use day_19::{parse_input, Comparison, Workflow};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    min: u64,
    max: u64,
}

fn get_num_combinations(ranges: &HashMap<char, Range>) -> u64 {
    ranges.values().map(|r| r.max - r.min).product()
}

fn get_accepted(
    workflows: &HashMap<String, Workflow>,
    start: &str,
    mut ranges: HashMap<char, Range>,
) -> u64 {
    if start == "A" {
        return get_num_combinations(&ranges);
    }

    if start == "R" {
        return 0;
    }

    let workflow = workflows.get(start).unwrap();

    let mut accepted = 0;
    for rule in &workflow.rules {
        match &rule.condition {
            Some(c) => {
                let mut new_ranges = ranges.clone();
                let range = ranges.get_mut(&c.property).unwrap();
                let new_range = match c.comparison {
                    // Min inclusive, max exclusive
                    Comparison::LessThan => Range {
                        min: range.min,
                        max: u64::min(range.max, c.number as u64),
                    },
                    Comparison::GreaterThan => Range {
                        min: u64::max(range.min, c.number as u64 + 1),
                        max: range.max,
                    },
                };

                if new_range.min > new_range.max {
                    continue;
                }

                new_ranges.insert(c.property, new_range.clone());
                accepted += get_accepted(workflows, &rule.workflow, new_ranges);

                // Reduce range for next rule
                if new_range.min != range.min {
                    range.max = new_range.min;
                } else if new_range.max != range.max {
                    range.min = new_range.max;
                }
            }
            None => accepted += get_accepted(workflows, &rule.workflow, ranges.clone()),
        }
    }

    accepted
}

fn process(input: &str) -> u64 {
    let system = parse_input(input);
    let accepted = get_accepted(
        &system.workflows,
        "in",
        system.parts[0]
            .properties
            .keys()
            // Min inclusive, max exclusive
            .map(|p| (*p, Range { min: 1, max: 4001 }))
            .collect(),
    );

    accepted
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 143760172569135);
    }
}
