// --- Day 15: Lens Library ---

use advent_of_code_2023::read_lines;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let sum = part_1("src/bin/day15/input.txt");
    println!("Part 1: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let sum = unsafe { part_2("src/bin/day15/input.txt") };
    println!("Part 2: {:?}", sum);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn part_1(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    calculate_sum_of_hashes(input)
}

unsafe fn part_2(filename: &str) -> u64 {
    let input = read_lines(filename).unwrap();
    let hashmap = create_boxes(input);
    calculate_focusing_powers(hashmap)
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

fn calculate_sum_of_hashes(input: Vec<String>) -> u64 {
    input
        .iter()
        .flat_map(|line| {
            line.split(',')
                .map(|seq| label_hash(&seq.as_bytes()) as u64)
        })
        .sum()
}

unsafe fn create_boxes(input: Vec<String>) -> Vec<HashMap<String, (String, usize)>> {
    let mut boxes: Vec<HashMap<String, (String, usize)>> = vec![HashMap::new(); 256];
    for line in &input {
        for seq in line.split(',') {
            let b = seq.as_bytes();
            if let Some(_value) = b[b.len() - 1].checked_sub(b'0') {
                let label = &b[0..b.len() - 2];
                let label_hash = label_hash(label) as usize;
                let len = boxes[label_hash].len() + 1;
                boxes[label_hash]
                    .entry(String::from_utf8_unchecked(label.to_vec()))
                    .and_modify(|v| *v = (seq.to_string(), v.1))
                    .or_insert((seq.to_string(), len));
            } else {
                let label = &b[0..b.len() - 1];
                let str_label = &String::from_utf8_unchecked(label.to_vec());
                let label_hash = label_hash(label) as usize;
                if boxes[label_hash].contains_key(str_label) {
                    let slot = boxes[label_hash][str_label].1;
                    boxes[label_hash].remove(str_label);
                    for (_key, value) in boxes[label_hash].iter_mut() {
                        if value.1 > slot {
                            value.1 -= 1;
                        }
                    }
                }
            }
        }
    }
    boxes
}

fn calculate_focusing_powers(boxes: Vec<HashMap<String, (String, usize)>>) -> u64 {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, map)| {
            map.iter().map(move |(_key, value)| {
                let b = value.0.as_bytes();
                let focal_length = b[b.len() - 1] - b'0';
                let mut focusing_power = i as u64 + 1;
                focusing_power *= value.1 as u64;
                focusing_power *= focal_length as u64;
                focusing_power
            })
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
        let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()];
        assert_eq!(calculate_sum_of_hashes(input), 1320);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day15/test_input.txt"), 1320);
    }

    #[test]
    fn test_create_boxes() {
        let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()];
        let boxes = unsafe { create_boxes(input) };
        assert_eq!(boxes[0]["rn"], ("rn=1".to_string(), 1));
        assert_eq!(boxes[0]["cm"], ("cm=2".to_string(), 2));
        assert_eq!(boxes[3]["ot"], ("ot=7".to_string(), 1));
        assert_eq!(boxes[3]["ab"], ("ab=5".to_string(), 2));
        assert_eq!(boxes[3]["pc"], ("pc=6".to_string(), 3));
        assert_eq!(boxes[0].len(), 2);
        assert_eq!(boxes[1].len(), 0);
        assert_eq!(boxes[2].len(), 0);
        assert_eq!(boxes[3].len(), 3);
        assert_eq!(boxes[255].len(), 0);
    }

    #[test]
    fn test_calculate_focusing_powers() {
        let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()];
        let hashmap = unsafe { create_boxes(input) };
        assert_eq!(calculate_focusing_powers(hashmap), 145);
    }

    #[test]
    fn test_part_2() {
        unsafe {
            assert_eq!(part_2("src/bin/day15/test_input.txt"), 145);
        }
    }
}
