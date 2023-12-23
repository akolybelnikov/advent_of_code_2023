// --- Day 20: Pulse Propagation ---

use advent_of_code_2023::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day20/input.txt", 1000);
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str, num: usize) -> usize {
    let input = read_lines(filename).unwrap();
    let mut circuit = Circuit::new(input);
    while circuit.count < num {
        circuit.broadcast();
    }
    let (high_count, low_count) = circuit.count_pulses();

    high_count * low_count
}

const BROADCASTER: &str = "broadcaster";
const OUTPUT: &str = "output";

struct Circuit {
    modules: HashMap<String, Module>,
    count: usize,
}

impl Circuit {
    fn new(input: Vec<String>) -> Circuit {
        let modules = make_modules_from_input(input);
        Circuit { modules, count: 0 }
    }
    fn broadcast(&mut self) {
        self.count += 1;
        let mut pulses = VecDeque::new();
        let broadcaster = self.modules.get_mut(BROADCASTER).unwrap();
        for propagation in broadcaster.propagate(Pulse::Low) {
            pulses.push_back(propagation);
        }
        while let Some(propagation) = pulses.pop_front() {
            if let Some(module) = self.modules.get_mut(&propagation.2) {
                if let Some(new_pulse) = module.pulse(&propagation.0, propagation.1) {
                    for next in module.propagate(new_pulse) {
                        pulses.push_back(next);
                    }
                }
            }
        }
    }

    fn count_pulses(&self) -> (usize, usize) {
        let high_count: usize = self.modules.values().map(|m| m.high_count).sum();
        let low_count: usize = self.modules.values().map(|m| m.low_count).sum();
        (high_count, low_count + self.count)
    }
}

struct Propagation(String, Pulse, String);

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    Conjunction,
    Output,
    Switch,
}

#[derive(PartialEq)]
enum ModuleState {
    High,
    Low,
    Off,
    On,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

struct Module {
    high_count: usize,
    low_count: usize,
    module_type: ModuleType,
    name: String,
    receivers: Vec<String>,
    senders: HashMap<String, Pulse>,
    state: ModuleState,
}

impl Module {
    fn new(config: &str) -> Module {
        let (module_name, receivers) = config.split(" -> ").collect_tuple().unwrap();
        let mut module = Module {
            high_count: 0,
            low_count: 0,
            module_type: ModuleType::Switch,
            name: module_name
                .trim_start_matches(|c| c == '%' || c == '&')
                .to_string(),
            receivers: Vec::new(),
            senders: HashMap::new(),
            state: ModuleState::Off,
        };
        for receiver in receivers.split(", ") {
            module.receivers.push(receiver.to_string());
        }
        match module_name.as_bytes()[0] {
            b'b' => module.module_type = ModuleType::Broadcaster,
            b'%' => module.module_type = ModuleType::Switch,
            b'&' => {
                module.module_type = ModuleType::Conjunction;
                module.state = ModuleState::Low;
            }
            _ => module.module_type = ModuleType::Output,
        }
        module
    }

    fn pulse(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match self.module_type {
            ModuleType::Conjunction => {
                self.senders.insert(from.to_string(), pulse);
                if self.senders.iter().all(|(_, &p)| p == Pulse::High) {
                    self.state = ModuleState::High;
                    return Some(Pulse::Low);
                } else if self.senders.iter().any(|(_, &p)| p == Pulse::Low) {
                    self.state = ModuleState::Low;
                    return Some(Pulse::High);
                }
            }
            ModuleType::Switch => {
                if pulse == Pulse::Low {
                    return if self.state == ModuleState::Off {
                        self.state = ModuleState::On;
                        Some(Pulse::High)
                    } else {
                        self.state = ModuleState::Off;
                        Some(Pulse::Low)
                    };
                }
            }
            _ => return None,
        }
        None
    }

    fn propagate(&mut self, pulse: Pulse) -> Vec<Propagation> {
        let mut pulses = Vec::new();
        for receiver in &self.receivers {
            if pulse == Pulse::High {
                self.high_count += 1;
            } else {
                self.low_count += 1;
            }
            if receiver != OUTPUT {
                pulses.push(Propagation(self.name.clone(), pulse, receiver.clone()));
            }
        }
        pulses
    }

    fn is_conjunction(&self) -> bool {
        self.module_type == ModuleType::Conjunction
    }

    fn is_switch(&self) -> bool {
        self.module_type == ModuleType::Switch
    }
}

fn make_modules_from_input(input: Vec<String>) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input {
        let module = Module::new(&line);
        modules.insert(module.name.clone(), module);
    }

    let mut temp = vec![];

    for module in modules.values_mut() {
        if module.is_switch() {
            for receiver in &module.receivers {
                temp.push((receiver.clone(), module.name.clone()));
            }
        }
    }

    for (receiver, module_name) in temp {
        if let Some(module) = modules.get_mut(&receiver) {
            if module.is_conjunction() {
                module.senders.insert(module_name.clone(), Pulse::Low);
            }
        } else {
            println!(
                "sender {} could not find receiver {}",
                module_name, receiver
            )
        }
    }

    modules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_new() {
        let module = Module::new("broadcaster -> a, b, c");
        assert_eq!(module.name, "broadcaster");
        assert_eq!(module.module_type, ModuleType::Broadcaster);
        assert_eq!(module.receivers, vec!["a", "b", "c"]);

        let module = Module::new("%a -> b");
        assert_eq!(module.name, "a");
        assert_eq!(module.module_type, ModuleType::Switch);
        assert_eq!(module.receivers, vec!["b"]);

        let module = Module::new("%b -> c");
        assert_eq!(module.name, "b");
        assert_eq!(module.module_type, ModuleType::Switch);
        assert_eq!(module.receivers, vec!["c"]);

        let module = Module::new("%c -> inv");
        assert_eq!(module.name, "c");
        assert_eq!(module.module_type, ModuleType::Switch);
        assert_eq!(module.receivers, vec!["inv"]);

        let module = Module::new("&inv -> a");
        assert_eq!(module.name, "inv");
        assert_eq!(module.module_type, ModuleType::Conjunction);
    }

    #[test]
    fn test_make_modules_from_input() {
        let input = vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ];
        let modules = make_modules_from_input(input);
        assert_eq!(modules.len(), 5);
        assert_eq!(
            modules.get("broadcaster").unwrap().receivers,
            vec!["a", "b", "c"]
        );
        assert_eq!(modules.get("a").unwrap().receivers, vec!["b"]);
        assert_eq!(modules.get("b").unwrap().receivers, vec!["c"]);
        assert_eq!(modules.get("c").unwrap().receivers, vec!["inv"]);
        assert_eq!(modules.get("inv").unwrap().receivers, vec!["a"]);
        assert_eq!(
            modules
                .get("inv")
                .unwrap()
                .senders
                .get_key_value("c")
                .unwrap(),
            (&"c".to_string(), &Pulse::Low)
        );
    }

    #[test]
    fn test_circuit() {
        let input = vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ];
        let mut circuit = Circuit::new(input);
        circuit.broadcast();
        let (high_count, low_count) = circuit.count_pulses();
        assert_eq!(high_count, 4);
        assert_eq!(low_count, 8);
    }

    #[test]
    fn test_circuit_2() {
        let input = vec![
            "broadcaster -> a".to_string(),
            "%a -> inv, con".to_string(),
            "&inv -> b".to_string(),
            "%b -> con".to_string(),
            "&con -> output".to_string(),
        ];
        let mut circuit = Circuit::new(input);
        circuit.broadcast();
        let (high_count, low_count) = circuit.count_pulses();
        assert_eq!(high_count, 4);
        assert_eq!(low_count, 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day20/test_input.txt", 1000), 32000000);
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(part_1("src/bin/day20/test_input_2.txt", 1000), 11687500);
    }
}
