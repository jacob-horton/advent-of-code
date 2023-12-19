use std::collections::HashMap;

use day_19::{parse_input, Comparison, Part, Workflow};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current_workflow = "in";

    while current_workflow != "A" && current_workflow != "R" {
        let workflow = workflows.get(current_workflow).unwrap();

        for rule in &workflow.rules {
            let condition_met = match &rule.condition {
                Some(c) => {
                    let num = part.properties.get(&c.property).unwrap();
                    match c.comparison {
                        Comparison::LessThan => num < &c.number,
                        Comparison::GreaterThan => num > &c.number,
                    }
                }
                None => true,
            };

            if condition_met {
                current_workflow = &rule.workflow;
                break;
            }
        }
    }

    current_workflow == "A"
}

fn sum_properties(part: &Part) -> u32 {
    part.properties.values().sum()
}

fn process(input: &str) -> u32 {
    let system = parse_input(input);

    system
        .parts
        .into_iter()
        .filter(|p| is_accepted(&p, &system.workflows))
        .map(|p| sum_properties(&p))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 19114);
    }
}
