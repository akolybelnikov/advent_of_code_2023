// --- Day 4: Scratchcards ---

use std::collections::HashMap;

#[derive(Default, Debug)]
struct Scratchcard {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    points: u32,
}

impl Scratchcard {
    // parse the input string like Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn new(input: &str) -> Scratchcard {
        let mut card = Scratchcard::default();
        let mut parts = input.split(":");
        card.id = parts
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut numbers = parts.next().unwrap().split("|");
        card.winning = parse_numbers(numbers.next().unwrap().trim());
        card.numbers = parse_numbers(numbers.next().unwrap().trim());
        card.calculate_points();
        card
    }

    fn calculate_points(&mut self) {
        self.points = self
            .winning
            .iter()
            .filter(|x| self.numbers.contains(x))
            .count() as u32
    }
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day04/input.txt"));
    println!("Part 2: {}", part_2("src/bin/day04/input.txt"));
}

fn part_1(filename: &str) -> i32 {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut total = 0;
    for line in lines {
        let card = Scratchcard::new(&line);
        total += if card.points > 0 {
            2_i32.pow(card.points - 1)
        } else {
            0
        };
    }
    total
}

fn part_2(filename: &str) -> u32 {
    let mut cards: HashMap<u32, u32> = HashMap::new();
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    for line in lines {
        let card = Scratchcard::new(&line);
        *cards.entry(card.id).or_insert(0) += 1;
        if card.points > 0 {
            for _i in 0..cards[&card.id] {
                for i in card.id + 1..=card.id + card.points {
                    *cards.entry(i).or_insert(0) += 1;
                }
            }
        }
    }
    cards.values().cloned().sum()
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("1 2 3 4 5"), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_card() {
        let card = Scratchcard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 ");
        assert_eq!(card.id, 1);
        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_winning_numbers() {
        let card = Scratchcard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 ");
        assert_eq!(card.points, 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day04/test_input.txt"), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day04/test_input.txt"), 30);
    }
}
