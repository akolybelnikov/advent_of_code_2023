// --- Day 19: Aplenty ---

use advent_of_code_2023::read_lines;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum Rule {
    WorkflowAction(WorkflowAction),
    ComplexRule {
        matcher: char,
        op: char,
        value: u64,
        outcome: String,
    },
}

#[derive(Debug, PartialEq)]
enum WorkflowAction {
    DoNothing,
    Approve,
    Reject,
    SendToWorkflow(String),
}

impl Rule {
    fn new(rule: &str) -> Result<Rule, std::num::ParseIntError> {
        match rule {
            "A" => Ok(Rule::WorkflowAction(WorkflowAction::Approve)),
            "R" => Ok(Rule::WorkflowAction(WorkflowAction::Reject)),
            _ => {
                let bytes = rule.as_bytes();
                match bytes.get(1) {
                    Some(&op @ b'<') | Some(&op @ b'>') => {
                        let matcher = bytes[0] as char;
                        let colon_offset = bytes.iter().position(|&x| x == b':').unwrap();
                        let value = std::str::from_utf8(&bytes[2..colon_offset])
                            .unwrap()
                            .parse::<u64>()?;
                        let outcome = std::str::from_utf8(&bytes[colon_offset + 1..])
                            .unwrap()
                            .to_string();
                        Ok(Rule::ComplexRule {
                            matcher,
                            op: op as char,
                            value,
                            outcome,
                        })
                    }
                    _ => Ok(Rule::WorkflowAction(WorkflowAction::SendToWorkflow(
                        rule.to_string(),
                    ))),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(part: &str) -> Part {
        let part = part.trim_start_matches('{').trim_end_matches('}');
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for sub in part.split(',') {
            let sub_parts: Vec<&str> = sub.split('=').collect();
            let key = sub_parts[0].trim();
            let value: u64 = sub_parts[1].trim().parse().unwrap();
            match key {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => panic!("Unexpected key"),
            }
        }
        Part { x, m, a, s }
    }

    fn sum(&self) -> i64 {
        self.x as i64 + self.m as i64 + self.a as i64 + self.s as i64
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(workflow: &str) -> Workflow {
        let workflow = workflow.split('{').collect::<Vec<&str>>();
        let name = workflow[0].to_string();
        let rules = workflow[1]
            .trim_end_matches('}')
            .split(',')
            .map(|rule| Rule::new(rule).unwrap())
            .collect::<Vec<Rule>>();
        Workflow { name, rules }
    }

    fn process_part(&self, part: Part) -> WorkflowAction {
        for rule in &self.rules {
            match rule {
                Rule::WorkflowAction(WorkflowAction::Approve) => return WorkflowAction::Approve,
                Rule::WorkflowAction(WorkflowAction::Reject) => return WorkflowAction::Reject,
                Rule::WorkflowAction(WorkflowAction::SendToWorkflow(workflow)) => {
                    return WorkflowAction::SendToWorkflow(workflow.to_string());
                }
                Rule::ComplexRule {
                    matcher,
                    op,
                    value,
                    outcome,
                } => {
                    let part_value = match matcher {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => panic!("Unexpected matcher"),
                    };
                    if compare_values(*op, part_value, *value) {
                        return match outcome.as_str() {
                            "A" => WorkflowAction::Approve,
                            "R" => WorkflowAction::Reject,
                            _ => WorkflowAction::SendToWorkflow(outcome.to_string()),
                        };
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
        WorkflowAction::DoNothing
    }
}

fn split_input(input_lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let empty_line_index = input_lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap();

    let workflow_lines = input_lines[..empty_line_index].to_vec();
    let part_lines = input_lines[empty_line_index + 1..].to_vec();
    (workflow_lines, part_lines)
}

fn create_workflows(workflow_lines: Vec<String>) -> HashMap<String, Workflow> {
    workflow_lines
        .iter()
        .map(|line| {
            let workflow = Workflow::new(line);
            (workflow.name.clone(), workflow)
        })
        .collect()
}

fn process_parts_1(input_lines: Vec<String>) -> i64 {
    let (workflow_lines, part_lines) = split_input(input_lines);

    let workflows = create_workflows(workflow_lines);

    let mut parts: VecDeque<(Part, String)> = part_lines
        .iter()
        .map(|line| (Part::new(line), "in".to_string()))
        .collect();
    let mut approved: Vec<Part> = vec![];

    while let Some((part, workflow_name)) = parts.pop_front() {
        let workflow = workflows.get(&workflow_name).unwrap();
        let action = workflow.process_part(part);
        match action {
            WorkflowAction::Approve => approved.push(part),
            WorkflowAction::SendToWorkflow(workflow_name) => parts.push_back((part, workflow_name)),
            _ => {}
        }
    }

    approved.iter().map(|part| part.sum()).sum()
}

fn compare_values(op: char, lhs: u64, rhs: u64) -> bool {
    match op {
        '<' => lhs < rhs,
        '>' => lhs > rhs,
        _ => panic!("Unsupported operator: {}", op),
    }
}

struct WorkflowRule {
    key: char,
    cmp: char,
    n: u64,
    target: String,
}

struct Workflow2 {
    rules: Vec<WorkflowRule>,
    fallback: String,
}

fn create_workflows_2(workflow_lines: Vec<String>) -> HashMap<String, Workflow2> {
    let mut workflows = HashMap::new();
    for line in workflow_lines {
        let parts: Vec<&str> = line.split('{').collect();
        let name = parts[0];
        let rules: Vec<&str> = parts[1].trim_end_matches('}').split(',').collect();
        let fallback = rules.last().unwrap().to_string();
        let mut wf_rules: Vec<WorkflowRule> = vec![];
        for i in 0..rules.len() - 1 {
            let rule = rules[i];
            let (cmp_part, target) = rule.split_at(rule.find(':').unwrap());
            let target = target.trim_start_matches(':').to_string();
            let key = cmp_part.chars().next().unwrap();
            let cmp = cmp_part.chars().nth(1).unwrap();
            let n = cmp_part[2..].parse::<u64>().unwrap();
            wf_rules.push(WorkflowRule {
                key,
                cmp,
                n,
                target,
            });
        }
        workflows.insert(
            name.to_string(),
            Workflow2 {
                rules: wf_rules,
                fallback,
            },
        );
    }
    workflows
}

fn count(
    ranges: &mut HashMap<char, (u64, u64)>,
    workflows: &HashMap<String, Workflow2>,
    wf_name: &str,
) -> u64 {
    match wf_name {
        "R" => 0,
        "A" => {
            let mut product = 1;
            for (_, (lo, hi)) in ranges.iter() {
                product *= hi - lo + 1;
            }
            product
        }
        _ => {
            let mut total = 0;
            let wf = workflows.get(wf_name).unwrap();
            for WorkflowRule {
                key,
                cmp,
                n,
                target,
            } in &wf.rules
            {
                let (lo, hi) = ranges.get(key).unwrap().clone();
                let (t, f) = match cmp {
                    '<' => ((lo, (*n - 1).min(hi)), ((*n).max(lo), hi)),
                    _ => (((*n + 1).max(lo), hi), (lo, (*n).min(hi))),
                };
                if t.0 <= t.1 {
                    let mut copy = ranges.clone();
                    copy.insert(*key, t);
                    total += count(&mut copy, &workflows, target);
                }
                if f.0 <= f.1 {
                    ranges.insert(*key, f);
                } else {
                    break;
                }
            }
            total += count(ranges, &workflows, &wf.fallback);
            total
        }
    }
}

fn main() {
    let time_start = std::time::Instant::now();
    let ratings = part_1("src/bin/day19/input.txt");
    println!(
        "Part 1: {:?} , Time: {}μs",
        ratings,
        time_start.elapsed().as_micros()
    );

    let time_start = std::time::Instant::now();
    let count = part_2(read_lines("src/bin/day19/input.txt").unwrap());
    println!(
        "Part 2: {:?} , Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> i64 {
    let input_lines = read_lines(filename).unwrap();
    process_parts_1(input_lines)
}

fn part_2(input_lines: Vec<String>) -> u64 {
    let mut ranges: HashMap<char, (u64, u64)> = ['x', 'm', 'a', 's']
        .iter()
        .map(|&k| (k, (1, 4000)))
        .collect();
    let (workflow_lines, _) = split_input(input_lines);
    let workflows = create_workflows_2(workflow_lines);
    count(&mut ranges, &workflows, "in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_new() {
        let rule = Rule::new("A").unwrap();
        assert_eq!(rule, Rule::WorkflowAction(WorkflowAction::Approve));

        let rule = Rule::new("R").unwrap();
        assert_eq!(rule, Rule::WorkflowAction(WorkflowAction::Reject));

        let rule = Rule::new("x<537:gd").unwrap();
        assert_eq!(
            rule,
            Rule::ComplexRule {
                matcher: 'x',
                op: '<',
                value: 537,
                outcome: "gd".to_string(),
            }
        );

        let rule = Rule::new("rfg").unwrap();
        assert_eq!(
            rule,
            Rule::WorkflowAction(WorkflowAction::SendToWorkflow("rfg".to_string()))
        );
    }

    #[test]
    fn test_part_new() {
        let part = Part::new("{x=0,m=0,a=0,s=0}");
        assert_eq!(part.x, 0);
        assert_eq!(part.m, 0);
        assert_eq!(part.a, 0);
        assert_eq!(part.s, 0);

        let part = Part::new("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(part.x, 787);
        assert_eq!(part.m, 2655);
        assert_eq!(part.a, 1222);
        assert_eq!(part.s, 2876);
    }

    #[test]
    fn test_workflow_new() {
        let workflow = Workflow::new("px{a<2006:qkq,m>2090:A,rfg}");
        assert_eq!(workflow.name, "px");
        assert_eq!(
            workflow.rules,
            vec![
                Rule::ComplexRule {
                    matcher: 'a',
                    op: '<',
                    value: 2006,
                    outcome: "qkq".to_string(),
                },
                Rule::ComplexRule {
                    matcher: 'm',
                    op: '>',
                    value: 2090,
                    outcome: "A".to_string(),
                },
                Rule::WorkflowAction(WorkflowAction::SendToWorkflow("rfg".to_string())),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let ratings = part_1("src/bin/day19/test_input.txt");
        assert_eq!(ratings, 19114);
    }

    #[test]
    fn test_process_parts_2() {
        let lines = read_lines("src/bin/day19/test_input.txt").unwrap();
        assert_eq!(part_2(lines), 167409079868000);
    }
}
