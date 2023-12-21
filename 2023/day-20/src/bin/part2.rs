use std::collections::{HashMap, VecDeque};

use day_20::{parse_input, Module, ModuleType};
use num::Integer;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

// These modules feed into a conjunction which feeds into "rx"
// Once all these pulse low, rx will be pulsed high
static LAST_MODULES: [&str; 4] = ["st", "tn", "hh", "dt"];

fn pulse_and_cache(
    modules: &mut HashMap<String, Module>,
    cache: &mut HashMap<String, u64>,
    i: u64,
) -> bool {
    let mut frontier = VecDeque::new();
    frontier.push_back(("button".to_string(), false, "broadcaster".to_string()));

    while let Some((prev, pulse, module_name)) = frontier.pop_front() {
        // If pulsing low, and module feeds into the conjunction before rx, cache the number of
        // button presses to get to this point
        if !pulse && LAST_MODULES.contains(&module_name.as_str()) {
            // Only cache first time - we want lowest number
            cache.entry(module_name.clone()).or_insert(i);
        }

        let Some(module) = modules.get_mut(&module_name) else {
            continue;
        };

        let next: Vec<(String, bool, String)> = match &module.module_type {
            ModuleType::Broadcast => module
                .outputs
                .clone()
                .into_iter()
                .map(|m| (module_name.clone(), pulse, m))
                .collect(),

            ModuleType::FlipFlop(state) => {
                if pulse {
                    continue;
                }

                let new_state = !state;
                module.module_type = ModuleType::FlipFlop(new_state);

                module
                    .outputs
                    .clone()
                    .into_iter()
                    .map(|m| (module_name.clone(), new_state, m))
                    .collect()
            }
            ModuleType::Conjunction(memory) => {
                let mut memory = memory.clone();
                memory.insert(prev, pulse);
                module.module_type = ModuleType::Conjunction(memory.clone());

                let new_pulse = !module
                    .inputs
                    .iter()
                    .all(|i| memory.entry(i.to_owned()).or_insert(false).to_owned());

                module
                    .outputs
                    .clone()
                    .into_iter()
                    .map(|m| (module_name.clone(), new_pulse, m))
                    .collect()
            }
        };

        frontier.append(&mut next.into_iter().collect());
    }

    false
}

fn process(input: &str) -> u64 {
    let mut modules = parse_input(input);
    let mut cache = HashMap::new();

    let mut count = 1;
    while !pulse_and_cache(&mut modules, &mut cache, count) {
        if LAST_MODULES
            .iter()
            .all(|m| cache.contains_key(m.to_owned()))
        {
            break;
        }

        count += 1;
    }

    // From testing, loops go exactly back to the start state, so the first time we encounter the
    // low pulse is also the length of the loop. This means we can just use LCM
    let diffs: Vec<u64> = cache.into_values().collect();
    let result = diffs.into_iter().reduce(|a, b| a.lcm(&b)).unwrap();

    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 233338595643977);
    }
}
