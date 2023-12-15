// --- Day 15: Lens Library ---

use std::time::Instant;
use advent_of_code_2023::read_lines;

const EQUALS: u8 = b'=';
const DASH: u8 = b'-';

fn main() {
    let time_start = Instant::now();
    let sum = part_1("src/bin/day15/input.txt");
    println!("Part 1: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    calculate_hash_sum(input)
}

fn label_hash(input: &[u8]) -> u32 {
    let mut sum = 0;
    for i in 0..input.len() {
        sum += input[i] as u32;
        sum *= 17;
        sum %= 256;
    }
    sum
}

fn calculate_hash_sum(input: Vec<String>) -> u64 {
    input
        .iter()
        .flat_map(|line| {
            line.split(',')
                .map(|seq| label_hash(&seq.as_bytes()) as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_step() {
        // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        assert_eq!(label_hash(&"rn=1".as_bytes().to_vec()), 30);
        assert_eq!(label_hash(&"cm-".as_bytes().to_vec()), 253);
        assert_eq!(label_hash(&"qp=3".as_bytes().to_vec()), 97);
        assert_eq!(label_hash(&"cm=2".as_bytes().to_vec()), 47);
        assert_eq!(label_hash(&"qp-".as_bytes().to_vec()), 14);
        assert_eq!(label_hash(&"pc=4".as_bytes().to_vec()), 180);
        assert_eq!(label_hash(&"ot=9".as_bytes().to_vec()), 9);
        assert_eq!(label_hash(&"ab=5".as_bytes().to_vec()), 197);
        assert_eq!(label_hash(&"pc-".as_bytes().to_vec()), 48);
        assert_eq!(label_hash(&"pc=6".as_bytes().to_vec()), 214);
        assert_eq!(label_hash(&"ot=7".as_bytes().to_vec()), 231);
    }

    #[test]
    fn test_sum_steps() {
        let input = vec![
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string(),
        ];
        assert_eq!(calculate_hash_sum(input), 1320);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day15/test_input.txt"), 1320);
    }
}
