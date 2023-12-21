fn main() {
    println!("Hello from day19!");
}

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
                let parts = rule.as_bytes();
                match parts.get(1) {
                    Some(&op @ b'<') | Some(&op @ b'>') => {
                        let matcher = parts[0] as char;
                        let colon_offset = parts.iter().position(|&x| x == b':').unwrap();
                        let value = std::str::from_utf8(&parts[2..colon_offset]).unwrap().parse::<i32>()?;
                        let outcome = std::str::from_utf8(&parts[colon_offset + 1..]).unwrap().to_string();
                        Ok(Rule::ComplexRule {
                            matcher,
                            op: op as char,
                            value,
                            outcome,
                        })
                    }
                    _ => Ok(Rule::SimpleAction(SimpleAction::SendToWorkflow(rule.to_string()))),
                }
            }
        }
    }
}

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
}

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
            .map(|r| Rule::new(r).unwrap())
            .collect::<Vec<Rule>>();
        Workflow { name, rules }
    }
}

// fn compare_values(op: &str, lhs: i32, rhs: i32) -> bool {
//     match op {
//         "<" => lhs < rhs,
//         ">" => lhs > rhs,
//         _ => panic!("Unsupported operator: {}", op),
//     }
// }

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
        assert_eq!(rule, Rule::SimpleAction(SimpleAction::SendToWorkflow("rfg".to_string())));
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
}