#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

struct Node {
    value: String,
    left: String,
    right: String,
}

fn parse_node(input: &str) -> Node {
    let mut split = input.split(" = ");
    let value = split.next().map(|s| s.to_string()).unwrap_or_default();

    let children = split
        .next()
        .map(|s| s.trim_start_matches('(').trim_end_matches(')').to_string())
        .unwrap_or_default();

    let mut children = children.split(",").map(|s| s.trim());
    let left = children.next().unwrap_or_default().to_owned();

    let right = children.next().unwrap_or_default().to_owned();

    Node { value, left, right }
}

fn parse_nodes(input: &Vec<String>) -> Vec<Node> {
    input.iter().map(|s| parse_node(s)).collect()
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

fn main() {
    println!("Hello from day8!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node() {
        let input = "AAA = (BBB, CCC)";
        let node = parse_node(input);
        assert_eq!(node.value, "AAA");
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
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
        assert_eq!(nodes[0].value, "AAA");
        assert_eq!(nodes[0].left, "BBB");
        assert_eq!(nodes[0].right, "CCC");
        assert_eq!(nodes[1].value, "BBB");
        assert_eq!(nodes[1].left, "AAA");
        assert_eq!(nodes[1].right, "ZZZ");
        assert_eq!(nodes[2].value, "ZZZ");
        assert_eq!(nodes[2].left, "ZZZ");
        assert_eq!(nodes[2].right, "ZZZ");
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
}
