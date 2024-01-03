use anyhow::{anyhow, bail, Result};
use num::integer::lcm;
use std::collections::{BTreeMap, VecDeque};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 819397964)?;
    aoc::run!(part_two(input), 252667369442479)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlipFlopState {
    Off,
    On,
}

#[derive(Debug, PartialEq, Eq)]
enum GateType {
    Passthrough,
    FlipFlop(FlipFlopState),
    Conjunction(BTreeMap<String, Pulse>),
}

#[derive(Debug)]
struct Gate {
    r#type: GateType,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Network {
    gates: BTreeMap<String, Gate>,
    low_pulse_count: usize,
    high_pulse_count: usize,
}

impl Network {
    fn push_button(&mut self, watched_gate: Option<&str>) -> bool {
        let mut watched = false;
        let mut worklist = VecDeque::new(); // (src, dest, pulse)
        self.low_pulse_count += 1; // initial low pulse from button to broadcaster
        for output in self.gates["broadcaster"].outputs.iter() {
            worklist.push_back(("broadcaster".to_string(), output.to_string(), Pulse::Low));
            self.low_pulse_count += 1; // initial low pulse from broadcaster to <dest>
        }
        while let Some((src, dest, pulse)) = worklist.pop_front() {
            if src == watched_gate.unwrap_or_default() && dest == "tg" && pulse == Pulse::High {
                watched = true;
            }
            let gate = self.gates.get_mut(&dest).unwrap();
            let pulse = match gate.r#type {
                GateType::Passthrough => Some(pulse),
                GateType::FlipFlop(ref mut state) => match pulse {
                    Pulse::Low => match state {
                        FlipFlopState::Off => {
                            *state = FlipFlopState::On;
                            Some(Pulse::High)
                        }
                        FlipFlopState::On => {
                            *state = FlipFlopState::Off;
                            Some(Pulse::Low)
                        }
                    },
                    Pulse::High => None,
                },
                GateType::Conjunction(ref mut inputs) => {
                    *inputs.get_mut(&src).unwrap() = pulse;
                    if inputs.values().all(|p| p == &Pulse::High) {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
            };
            if let Some(pulse) = pulse {
                let count = match pulse {
                    Pulse::Low => &mut self.low_pulse_count,
                    Pulse::High => &mut self.high_pulse_count,
                };
                *count += gate.outputs.len();
                for output in gate.outputs.iter() {
                    worklist.push_back((dest.to_string(), output.to_string(), pulse));
                }
            }
        }
        watched
    }

    #[allow(dead_code)]
    fn graphviz(&self) {
        let mut s = String::new();
        s.push_str("digraph G {\n");
        s.push_str("node [style=\"filled\"];");
        for (name, gate) in self.gates.iter() {
            match gate.r#type {
                GateType::Passthrough => {
                    s.push_str(&format!(
                        "\"{}\" [label=\"%{}\" fillcolor=\"#25ace6\"];\n",
                        name, name
                    ));
                }
                GateType::FlipFlop(_) => {
                    s.push_str(&format!("\"{}\" [fillcolor=\"#fbfbfb\"];\n", name));
                }
                GateType::Conjunction(_) => {
                    s.push_str(&format!(
                        "\"{}\" [label=\"&{}\" fillcolor=\"#ffd428\"];\n",
                        name, name
                    ));
                }
            }
            for output in gate.outputs.iter() {
                s.push_str(&format!("\"{}\" -> \"{}\";\n", name, output));
            }
        }
        s.push_str("}\n");
        std::fs::write("input.dot", s).unwrap();
    }
}

fn parse(input: &str) -> Result<Network> {
    let mut gates = BTreeMap::new();
    for line in input.lines() {
        let (name, outputs) = line
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("missing arrow: \"{line}\""))?;

        let (r#type, name) = match name {
            "broadcaster" => (GateType::Passthrough, String::from(name)),
            x if x.starts_with('%') => (
                GateType::FlipFlop(FlipFlopState::Off),
                String::from(&name[1..]),
            ),
            x if x.starts_with('&') => (
                GateType::Conjunction(BTreeMap::new()),
                String::from(&name[1..]),
            ),
            _ => bail!("unexpected name {name}"),
        };

        let outputs = outputs.split(", ").map(String::from).collect::<Vec<_>>();

        let gate = Gate { r#type, outputs };
        gates.insert(name, gate);
    }
    let mut sources: BTreeMap<String, Vec<_>> = BTreeMap::new();
    for (tx, gate) in gates.iter() {
        for rx in gate.outputs.iter() {
            sources
                .entry(rx.to_string())
                .or_default()
                .push(tx.to_string());
        }
    }
    for (name, sources) in sources.into_iter() {
        let gate = gates.entry(name).or_insert_with(|| Gate {
            r#type: GateType::Passthrough,
            outputs: vec![],
        });
        if let GateType::Conjunction(ref mut inputs) = gate.r#type {
            for src in sources {
                inputs.insert(src, Pulse::Low);
            }
        }
    }
    Ok(Network {
        gates,
        low_pulse_count: 0,
        high_pulse_count: 0,
    })
}

fn part_one(input: &str) -> Result<usize> {
    let mut network = parse(input)?;
    for _ in 0..1000 {
        network.push_button(None);
    }
    Ok(network.low_pulse_count * network.high_pulse_count)
}

fn part_two(input: &str) -> Result<usize> {
    // See input.dot and input.png; the following is true for the input graph:
    //
    // - broadcast and rx are the only two Passthrough gates
    // - rx is immediately preceded by a number of Conjunction gates; no other Conjunction gates
    //   exist
    // - all other gates are FlipFlop gates
    //
    // Conclusion: the input to rx is four Conjunction gates:
    //
    // - &ln, &db, &tf, &vq
    //
    // When these are all high, rx will be low.
    //
    // Assumption: &ln, &db, ... form individual cycles, and neither require "warm-up" to find the
    // cycle frequency. Calculating the LCM for the four cycles will yield the answer to the
    // puzzle.
    fn find_min_button_presses(input: &str, watched_gate: &str) -> Result<usize> {
        let mut network = parse(input)?;
        for i in 1.. {
            if network.push_button(Some(watched_gate)) {
                return Ok(i);
            }
        }
        panic!("won't happen");
    }

    let ln = find_min_button_presses(input, "ln")?;
    let db = find_min_button_presses(input, "db")?;
    let tf = find_min_button_presses(input, "tf")?;
    let vq = find_min_button_presses(input, "vq")?;
    Ok(lcm(ln, lcm(db, lcm(tf, vq))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str = include_str!("test-input-a.txt");
    const INPUT_B: &str = include_str!("test-input-b.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_A).unwrap(), 32_000_000);
        assert_eq!(part_one(INPUT_B).unwrap(), 11_687_500);
    }
}
