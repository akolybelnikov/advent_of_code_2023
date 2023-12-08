// --- Day 8: Haunted Wasteland ---
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

fn parse_node(input: &str) -> (String, Vec<String>) {
    let mut split = input.split(" = ");
    let value = split.next().map(|s| s.to_string()).unwrap_or_default();

    let children = split
        .next()
        .map(|s| s.trim_start_matches('(').trim_end_matches(')').to_string())
        .unwrap_or_default();

    let mut children = children.split(",").map(|s| s.trim());
    let left = children.next().unwrap_or_default().to_owned();

    let right = children.next().unwrap_or_default().to_owned();

    (value, vec![left, right])
}

fn parse_nodes(input: &[String]) -> HashMap<String, Vec<String>> {
    input
        .iter()
        .map(|s| parse_node(s))
        .collect::<HashMap<_, _>>()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect()
}

fn walk_tree<F>(
    nodes: HashMap<String, Vec<String>>,
    instructions: &Vec<Instruction>,
    start_node: String,
    end_fn: F,
) -> i64
where
    F: Fn(&String) -> bool,
{
    let mut current_node = start_node;
    let mut steps = 0;
    'outer: loop {
        for instruction in instructions.iter().cycle() {
            steps += 1;
            match instruction {
                Instruction::Left => {
                    let next_node = nodes[&current_node][0].clone();
                    current_node = next_node;
                }
                Instruction::Right => {
                    let next_node = nodes[&current_node][1].clone();
                    current_node = next_node;
                }
            }
            if end_fn(&current_node) {
                break 'outer;
            }
        }
    }
    steps
}

fn find_start_nodes(nodes: &HashMap<String, Vec<String>>) -> Vec<String> {
    nodes
        .iter()
        .filter(|(v, _)| v.as_bytes()[2] == b'A')
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn walk_tree_mul(nodes: HashMap<String, Vec<String>>, instructions: &Vec<Instruction>) -> i64 {
    let start_nodes = find_start_nodes(&nodes);
    let steps: Vec<i64> = start_nodes
        .iter()
        .map(|n| {
            walk_tree(nodes.clone(), instructions, n.clone(), |n| {
                n.as_bytes()[2] == b'Z'
            })
        })
        .collect();
    let steps = steps.iter().map(Clone::clone).reduce(lcm).unwrap_or(0);

    steps
}

fn part_1(filename: &str) -> i64 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let nodes = parse_nodes(&lines[2..]);
    let instructions = parse_instructions(&lines[0]);
    walk_tree(nodes, &instructions, "AAA".to_string(), |n| n == "ZZZ")
}

fn part_2(filename: &str) -> i64 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let nodes = parse_nodes(&lines[2..]);
    let instructions = parse_instructions(&lines[0]);
    walk_tree_mul(nodes, &instructions)
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day08/input.txt"));
    println!("Part 2: {}", part_2("src/bin/day08/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node() {
        let input = "AAA = (BBB, CCC)";
        let node = parse_node(input);
        assert_eq!(node.0, "AAA");
        assert_eq!(node.1[0], "BBB");
        assert_eq!(node.1[1], "CCC");
    }

    #[test]
    fn test_parse_nodes() {
        let input = vec![
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        let nodes = parse_nodes(&input);
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes["AAA"][0], "BBB");
        assert_eq!(nodes["AAA"][1], "CCC");
        assert_eq!(nodes["BBB"][0], "AAA");
        assert_eq!(nodes["BBB"][1], "ZZZ");
        assert_eq!(nodes["ZZZ"][0], "ZZZ");
        assert_eq!(nodes["ZZZ"][1], "ZZZ");
    }

    #[test]
    fn test_parse_instructions() {
        let input = "LLRRL";
        let instructions = parse_instructions(input);
        assert_eq!(instructions.len(), 5);
        assert_eq!(instructions[0], Instruction::Left);
        assert_eq!(instructions[1], Instruction::Left);
        assert_eq!(instructions[2], Instruction::Right);
        assert_eq!(instructions[3], Instruction::Right);
        assert_eq!(instructions[4], Instruction::Left);
    }

    #[test]
    fn test_walk_tree_6() {
        let input = vec![
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        let nodes = parse_nodes(&input);
        let instructions = parse_instructions("LLR");
        let result = walk_tree(nodes, &instructions, "AAA".to_string(), |n| n == "ZZZ");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_walk_tree_2() {
        let input = vec![
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        let nodes = parse_nodes(&input);
        assert_eq!(nodes.len(), 7);
        let instructions = parse_instructions("RL");
        let result = walk_tree(nodes, &instructions, "AAA".to_string(), |n| n == "ZZZ");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day08/test_input_1.txt"), 2);
        assert_eq!(part_1("src/bin/day08/test_input_2.txt"), 6);
    }

    #[test]
    fn test_find_start_nodes() {
        let input = vec![
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCA = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        let nodes = parse_nodes(&input);
        let start_nodes = find_start_nodes(&nodes);
        assert_eq!(start_nodes.len(), 2);
        assert_eq!(start_nodes.contains(&"AAA".to_string()), true);
        assert_eq!(start_nodes.contains(&"CCA".to_string()), true);
    }

    #[test]
    fn test_walk_tree_mul() {
        let input = vec![
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];
        let nodes = parse_nodes(&input);
        let instructions = parse_instructions("LR");
        let result = walk_tree_mul(nodes, &instructions);
        assert_eq!(result, 6);
    }
}
