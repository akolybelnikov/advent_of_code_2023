use crate::Rank::HighCard;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard = 1,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARD_LABELS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn card_labels() -> HashMap<char, u8> {
    let mut map = HashMap::new();
    let mut value = 2;

    for &card in CARD_LABELS.iter() {
        map.insert(card, value);
        value += 1;
    }

    map
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    bid: u32,
    cards: Vec<char>,
    rank: Rank,
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Hand {
        let mut hand = Hand {
            bid,
            cards,
            rank: HighCard,
        };
        hand.rank = hand.get_rank();
        hand
    }

    fn get_rank(&mut self) -> Rank {
        let mut counts: HashMap<char, u8> = HashMap::new();
        for card in &self.cards {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<(char, u8)> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let mut rank = HighCard;
        match counts[0].1 {
            5 => rank = Rank::FiveOfAKind,
            4 => rank = Rank::FourOfAKind,
            3 => {
                if counts[1].1 == 2 {
                    rank = Rank::FullHouse
                } else {
                    rank = Rank::ThreeOfAKind
                }
            }
            2 => {
                if counts[1].1 == 2 {
                    rank = Rank::TwoPairs
                } else {
                    rank = Rank::OnePair
                }
            }
            _ => (),
        }
        rank
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                let labels = card_labels();
                for i in 0..self.cards.len() {
                    if labels[&self.cards[i]] > labels[&other.cards[i]] {
                        return Ordering::Greater;
                    } else if labels[&self.cards[i]] < labels[&other.cards[i]] {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
            _ => self.rank.cmp(&other.rank),
        }
    }
}

fn create_hand_from_string(line: &str) -> Hand {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    let cards: Vec<char> = split_line[0].chars().collect();
    let bid: u32 = split_line[1].parse().unwrap();

    Hand::new(cards, bid)
}

fn hands_from_input(filename: &str) -> Vec<Hand> {
    advent_of_code_2023::read_lines(filename)
        .unwrap()
        .iter()
        .map(|line| create_hand_from_string(line))
        .collect()
}

fn part_1(filename: &str) -> u64 {
    let mut total = 0;
    let mut hands = hands_from_input(filename);
    hands.sort();
    // iterate with index to get the highest ranked hand
    for (i, hand) in hands.iter().enumerate() {
        total += (hand.bid * (i + 1) as u32) as u64;
    }
    total
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day07/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_new() {
        let hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0);
        assert_eq!(hand.cards, vec!['2', '3', '4', '5', '6']);
        assert_eq!(hand.rank, Rank::HighCard);
    }

    #[test]
    fn test_hand_get_rank() {
        let mut hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::HighCard);
        hand = Hand::new(vec!['2', '2', '2', '2', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::FourOfAKind);
        hand = Hand::new(vec!['2', '2', '2', '6', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::FullHouse);
        hand = Hand::new(vec!['2', '2', '2', '3', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::ThreeOfAKind);
        hand = Hand::new(vec!['2', '2', '3', '3', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::TwoPairs);
        hand = Hand::new(vec!['2', '2', '3', '4', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::OnePair);
        hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0);
        assert_eq!(hand.get_rank(), Rank::HighCard);
    }

    #[test]
    fn test_hand_cmp() {
        let mut hands = vec![
            Hand::new(vec!['3', '2', 'T', '3', 'K'], 765),
            Hand::new(vec!['T', '5', '5', 'J', '5'], 684),
            Hand::new(vec!['K', 'K', '6', '7', '7'], 28),
            Hand::new(vec!['K', 'T', 'J', 'J', 'T'], 220),
            Hand::new(vec!['Q', 'Q', 'Q', 'J', 'A'], 483),
        ];
        hands.sort();
        assert_eq!(hands[0].cards, vec!['3', '2', 'T', '3', 'K']);
        assert_eq!(hands[1].cards, vec!['K', 'T', 'J', 'J', 'T']);
        assert_eq!(hands[2].cards, vec!['K', 'K', '6', '7', '7']);
        assert_eq!(hands[3].cards, vec!['T', '5', '5', 'J', '5']);
        assert_eq!(hands[4].cards, vec!['Q', 'Q', 'Q', 'J', 'A']);
    }

    #[test]
    fn test_hands_from_input() {
        let hands = hands_from_input("src/bin/day07/test_input.txt");
        assert_eq!(hands[0].cards, vec!['3', '2', 'T', '3', 'K']);
        assert_eq!(hands[1].cards, vec!['T', '5', '5', 'J', '5']);
        assert_eq!(hands[2].cards, vec!['K', 'K', '6', '7', '7']);
        assert_eq!(hands[3].cards, vec!['K', 'T', 'J', 'J', 'T']);
        assert_eq!(hands[4].cards, vec!['Q', 'Q', 'Q', 'J', 'A']);
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 684);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 220);
        assert_eq!(hands[4].bid, 483);
        assert_eq!(hands[0].rank, Rank::OnePair);
        assert_eq!(hands[1].rank, Rank::ThreeOfAKind);
        assert_eq!(hands[2].rank, Rank::TwoPairs);
        assert_eq!(hands[3].rank, Rank::TwoPairs);
        assert_eq!(hands[4].rank, Rank::ThreeOfAKind);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day07/test_input.txt"), 6440);
    }
}
