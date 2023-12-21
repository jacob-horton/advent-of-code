use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleType {
    // On or off
    FlipFlop(bool),

    // Most recent pulse from each input (false = low, true = high)
    Conjunction(BTreeMap<String, bool>),

    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub module_type: ModuleType,
}

pub fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    for line in input.trim().split('\n') {
        let (module, outputs) = line.split_once(" -> ").unwrap();

        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        match module.chars().next().unwrap() {
            // Broadcaster
            'b' => {
                modules.insert(
                    module.to_string(),
                    Module {
                        module_type: ModuleType::Broadcast,
                        inputs: Vec::new(),
                        outputs,
                    },
                );
            }
            '%' => {
                modules.insert(
                    module[1..].to_string(),
                    Module {
                        module_type: ModuleType::FlipFlop(false),
                        inputs: Vec::new(),
                        outputs,
                    },
                );
            }
            '&' => {
                modules.insert(
                    module[1..].to_string(),
                    Module {
                        module_type: ModuleType::Conjunction(BTreeMap::new()),
                        inputs: Vec::new(),
                        outputs,
                    },
                );
            }
            _ => panic!("Unknown module type"),
        }
    }

    // Work out all inputs
    for (module_name, module) in modules.clone() {
        for output in module.outputs {
            if let Some(o) = modules.get_mut(&output) {
                o.inputs.push(module_name.clone());
            }
        }
    }

    modules
}
