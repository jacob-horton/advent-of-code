use std::collections::{HashMap, VecDeque};

use day_20::{parse_input, Module, ModuleType};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// (low, high)
fn pulse_count(modules: &mut HashMap<String, Module>) -> (u32, u32) {
    let mut low = 0;
    let mut high = 0;

    let mut frontier = VecDeque::new();
    frontier.push_back(("button".to_string(), false, "broadcaster".to_string()));

    while let Some((prev, pulse, module_name)) = frontier.pop_front() {
        let module = modules.get_mut(&module_name);

        if pulse {
            high += 1;
        } else {
            low += 1;
        }

        let Some(module) = modules.get_mut(&module_name) else {
            continue;
        };

        match &module.module_type {
            ModuleType::Broadcast => frontier.append(
                &mut module
                    .outputs
                    .clone()
                    .into_iter()
                    .map(|m| (module_name.clone(), pulse, m))
                    .collect(),
            ),
            ModuleType::FlipFlop(state) => {
                if pulse {
                    continue;
                }

                let new_state = !state;
                module.module_type = ModuleType::FlipFlop(new_state);
                frontier.append(
                    &mut module
                        .outputs
                        .clone()
                        .into_iter()
                        .map(|m| (module_name.clone(), new_state.to_owned(), m))
                        .collect(),
                );
            }
            ModuleType::Conjunction(memory) => {
                let mut memory = memory.clone();
                memory.insert(prev, pulse);
                module.module_type = ModuleType::Conjunction(memory.clone());

                let new_pulse = !module
                    .inputs
                    .iter()
                    .all(|i| memory.entry(i.to_owned()).or_insert(false).to_owned());

                frontier.append(
                    &mut module
                        .outputs
                        .clone()
                        .into_iter()
                        .map(|m| (module_name.clone(), new_pulse, m))
                        .collect(),
                );
            }
        }
    }

    (low, high)
}

fn process(input: &str) -> u32 {
    let mut modules = parse_input(input);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let (l, h) = pulse_count(&mut modules);
        low += l;
        high += h;
    }

    low * high
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_input_2() {
        let input = include_str!("../inputs/test_part1_2.txt");
        let result = process(input);
        assert_eq!(result, 11687500);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 812721756);
    }
}
