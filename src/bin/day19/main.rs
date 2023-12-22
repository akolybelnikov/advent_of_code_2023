use advent_of_code_2023::read_lines;
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq)]
enum Rule {
    SimpleAction(SimpleAction),
    ComplexRule {
        matcher: char,
        op: char,
        value: i32,
        outcome: String,
    },
}

#[derive(Debug, PartialEq)]
enum SimpleAction {
    DoNothing,
    Approve,
    Reject,
    SendToWorkflow(String),
}

impl Rule {
    fn new(rule: &str) -> Result<Rule, std::num::ParseIntError> {
        match rule {
            "A" => Ok(Rule::SimpleAction(SimpleAction::Approve)),
            "R" => Ok(Rule::SimpleAction(SimpleAction::Reject)),
            _ => {
                // s<537:gd this is example of input
                let bytes = rule.as_bytes();
                match bytes.get(1) {
                    Some(&op @ b'<') | Some(&op @ b'>') => {
                        let matcher = bytes[0] as char;
                        let colon_offset = bytes.iter().position(|&x| x == b':').unwrap();
                        let value = std::str::from_utf8(&bytes[2..colon_offset])
                            .unwrap()
                            .parse::<i32>()?;
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
                    _ => Ok(Rule::SimpleAction(SimpleAction::SendToWorkflow(
                        rule.to_string(),
                    ))),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
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
            let value: i32 = sub_parts[1].trim().parse().unwrap();
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

    fn process_part(&self, part: Part) -> SimpleAction {
        for rule in &self.rules {
            match rule {
                Rule::SimpleAction(SimpleAction::Approve) => return SimpleAction::Approve,
                Rule::SimpleAction(SimpleAction::Reject) => return SimpleAction::Reject,
                Rule::SimpleAction(SimpleAction::SendToWorkflow(workflow)) => {
                    return SimpleAction::SendToWorkflow(workflow.to_string())
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
                        return SimpleAction::SendToWorkflow(outcome.to_string());
                    } else {
                        continue;
                    }
                }
                _ => {}
            }
        }
        SimpleAction::DoNothing
    }
}

async fn process_parts(input_lines: Vec<String>) -> i64 {
    let empty_line_index = input_lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap();

    let workflow_lines = input_lines[..empty_line_index].to_vec();
    let part_lines = input_lines[empty_line_index + 1..].to_vec();

    let workflows: Arc<HashMap<String, Workflow>> = Arc::new(
        workflow_lines
            .iter()
            .map(|line| {
                let workflow = Workflow::new(line);
                (workflow.name.clone(), workflow)
            })
            .collect(),
    );

    let parts: Vec<Part> = part_lines.iter().map(|line| Part::new(line)).collect();
    let accepted: Arc<Mutex<Vec<Part>>> = Arc::new(Mutex::new(Vec::new()));

    let (tx, rx) = tokio::sync::broadcast::channel::<(Part, String)>(100);
    let mut worker_handles = vec![];

    for part in parts {
        println!("Sending part {:?} to workflow 'in'", part); // Debug print
        tx.send((part, "in".to_string())).unwrap();
    }

    for _i in 1..5 {
        let workflows = Arc::clone(&workflows);
        let accepted = Arc::clone(&accepted);
        let mut task_receiver = tx.subscribe();
        let tx_clone = tx.clone();

        let handle = tokio::spawn(async move {
            while let Ok((part, workflow_name)) = task_receiver.recv().await {
                println!("Received part {:?} for workflow {:?}", part, workflow_name); // Debug print
                let workflow = workflows.get(&workflow_name).unwrap();

                let action = process_part(part, workflow).await;
                match action {
                    SimpleAction::Approve => {
                        let mut accepted = accepted.lock().await;
                        accepted.push(part);
                        println!("Approved part {:?}", part); // Debug print
                    }
                    SimpleAction::Reject => {
                        println!("Rejected part {:?}", part); // Debug print
                    }
                    SimpleAction::SendToWorkflow(workflow_name) => {
                        println!("Sending part {:?} to workflow {:?}", part, workflow_name); // Debug print
                        tx_clone.send((part, workflow_name)).unwrap();
                    }
                    _ => {}
                }
            }
        });
        worker_handles.push(handle);
    }

    drop(rx);
    join_all(worker_handles).await;

    let data = Arc::try_unwrap(accepted).unwrap().into_inner();

    data.iter().map(|part| part.sum()).sum()
}

async fn process_part(part: Part, workflow: &Workflow) -> SimpleAction {
    workflow.process_part(part)
}

fn compare_values(op: char, lhs: i32, rhs: i32) -> bool {
    match op {
        '<' => lhs < rhs,
        '>' => lhs > rhs,
        _ => panic!("Unsupported operator: {}", op),
    }
}

#[tokio::main]
async fn main() {
    let time_start = std::time::Instant::now();
    let ratings = part_1("src/bin/day19/input.txt").await;
    println!(
        "Part 1: {:?} , Time: {}μs",
        ratings,
        time_start.elapsed().as_micros()
    );
}

async fn part_1(filename: &str) -> i64 {
    let input_lines = read_lines(filename).unwrap();
    process_parts(input_lines).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_new() {
        let rule = Rule::new("A").unwrap();
        assert_eq!(rule, Rule::SimpleAction(SimpleAction::Approve));

        let rule = Rule::new("R").unwrap();
        assert_eq!(rule, Rule::SimpleAction(SimpleAction::Reject));

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
            Rule::SimpleAction(SimpleAction::SendToWorkflow("rfg".to_string()))
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
                Rule::SimpleAction(SimpleAction::SendToWorkflow("rfg".to_string())),
            ]
        );
    }

    #[tokio::test]
    async fn test_part_1() {
        let ratings = part_1("src/bin/day19/test_input.txt").await;
        assert_eq!(ratings, 0);
    }
}
