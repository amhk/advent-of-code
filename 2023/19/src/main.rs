use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut, RangeInclusive},
};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 421983)?;
    aoc::run!(part_two(input), 129249871135292)?;
    Ok(())
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl Index<&Field> for PartRange {
    type Output = RangeInclusive<usize>;

    fn index(&self, index: &Field) -> &Self::Output {
        match index {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

impl IndexMut<&Field> for PartRange {
    fn index_mut(&mut self, index: &Field) -> &mut Self::Output {
        match index {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }
}

#[derive(Debug)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Op {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Rule {
    Rule {
        label: String,
        field: Field,
        op: Op,
        value: usize,
    },
    Default {
        label: String,
    },
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

fn parse(input: &str) -> Result<(BTreeMap<String, Workflow>, Vec<Part>)> {
    let re_workflow = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();
    let re_rule = Regex::new(r"([xmas])(<|>)(\d+):(.*)").unwrap();
    let re_part = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let (input_rules, input_parts) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing blank line"))?;

    let mut workflows = BTreeMap::new();
    for line in input_rules.lines() {
        let (_, [label, rules_input]) = re_workflow
            .captures(line)
            .map(|caps| caps.extract())
            .ok_or_else(|| anyhow!("regex found no matches in \"{line}\""))?;
        let mut rules_input = rules_input.split(',').collect::<Vec<_>>();
        let default = rules_input
            .pop()
            .ok_or_else(|| anyhow!("empty conditional"))?;
        let mut rules = vec![];
        for rule_input in rules_input {
            let (field, op, value, label) = aoc::parse!(
                &re_rule,
                rule_input,
                |s| match s {
                    "x" => Ok(Field::X),
                    "m" => Ok(Field::M),
                    "a" => Ok(Field::A),
                    "s" => Ok(Field::S),
                    _ => Err(anyhow!("unexpected pattern")),
                },
                |s| match s {
                    "<" => Ok(Op::LessThan),
                    ">" => Ok(Op::GreaterThan),
                    _ => Err(anyhow!("unexpected pattern")),
                },
                |s| s.parse::<usize>(),
                |s| Ok::<String, &str>(s.to_string())
            )?;
            rules.push(Rule::Rule {
                label,
                field,
                op,
                value,
            });
        }
        rules.push(Rule::Default {
            label: default.to_string(),
        });
        workflows.insert(label.to_string(), Workflow { rules });
    }

    let mut parts = vec![];
    for line in input_parts.lines() {
        let (x, m, a, s) = aoc::parse4(
            &re_part,
            line,
            |s| s.parse::<usize>(),
            |s| s.parse::<usize>(),
            |s| s.parse::<usize>(),
            |s| s.parse::<usize>(),
        )?;
        parts.push(Part { x, m, a, s });
    }

    Ok((workflows, parts))
}

fn part_one(input: &str) -> Result<usize> {
    fn eval(rule: &Rule, part: &Part) -> bool {
        match rule {
            Rule::Rule {
                field, op, value, ..
            } => {
                let field = match field {
                    Field::X => &part.x,
                    Field::M => &part.m,
                    Field::A => &part.a,
                    Field::S => &part.s,
                };
                match op {
                    Op::LessThan => field < value,
                    Op::GreaterThan => field > value,
                }
            }
            Rule::Default { .. } => true,
        }
    }

    let (workflows, parts) = parse(input)?;
    let mut accepted: Vec<Part> = vec![];
    for part in parts {
        let mut label = "in".to_string();
        while label != "A" && label != "R" {
            let workflow = &workflows[&label];
            for rule in workflow.rules.iter() {
                if eval(rule, &part) {
                    label = match rule {
                        Rule::Rule { label, .. } => label.to_string(),
                        Rule::Default { label } => label.to_string(),
                    };
                    break;
                }
            }
        }
        if label == "A" {
            accepted.push(part);
        }
    }
    Ok(accepted
        .into_iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum())
}

fn part_two(input: &str) -> Result<usize> {
    fn scan(
        workflows: &BTreeMap<String, Workflow>,
        label: &str,
        mut part_range: PartRange,
        accepted: &mut Vec<PartRange>,
    ) {
        if label == "A" {
            accepted.push(part_range.clone());
            return;
        } else if label == "R" {
            return;
        }

        let workflow = &workflows[label];
        for rule in workflow.rules.iter() {
            match rule {
                Rule::Rule {
                    label,
                    field,
                    op,
                    value,
                } => match op {
                    Op::LessThan => {
                        if part_range[field].start() < value {
                            let mut clone = part_range.clone();
                            clone[field] = *part_range[field].start()..=*value - 1;
                            part_range[field] = *value..=*part_range[field].end();
                            scan(workflows, label, clone, accepted);
                        }
                    }
                    Op::GreaterThan => {
                        if part_range[field].end() > value {
                            let mut clone = part_range.clone();
                            clone[field] = *value + 1..=*part_range[field].end();
                            part_range[field] = *part_range[field].start()..=*value;
                            scan(workflows, label, clone, accepted);
                        }
                    }
                },
                Rule::Default { label } => {
                    scan(workflows, label, part_range.clone(), accepted);
                }
            }
        }
    }

    fn range(range: &RangeInclusive<usize>) -> usize {
        range.start().abs_diff(*range.end()) + 1
    }

    let (workflows, _) = parse(input)?;
    let mut accepted = vec![];
    scan(&workflows, "in", PartRange::new(), &mut accepted);
    Ok(accepted
        .into_iter()
        .map(|pr| range(&pr.x) * range(&pr.m) * range(&pr.a) * range(&pr.s))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 19114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 167409079868000);
    }
}
