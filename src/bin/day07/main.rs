use crate::Rank::HighCard;
use lazy_static::lazy_static;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;

type HandBuilder = fn(&String) -> Hand;

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

#[derive(Debug, Eq, PartialEq)]
enum Labels {
    Normal,
    WithJoker,
}

const CARD_LABELS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_LABELS_WITH_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

lazy_static! {
    static ref CARD_LABELS_MAP: HashMap<char, u8> = card_labels(CARD_LABELS, 2);
    static ref CARD_LABELS_WITH_JOCKER_MAP: HashMap<char, u8> =
        card_labels(CARD_LABELS_WITH_JOKER, 1);
}

fn card_labels(labels: [char; 13], start_value: u8) -> HashMap<char, u8> {
    let mut map = HashMap::new();
    let mut value = start_value;

    for &card in labels.iter() {
        map.insert(card, value);
        value += 1;
    }

    map
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    bid: u32,
    cards: Vec<char>,
    labels: Labels,
    rank: Rank,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.labels == Labels::WithJoker && other.labels == Labels::WithJoker {
            self.cmp_with_joker(other)
        } else {
            self.cmp_without_joker(other)
        };
    }
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32, labels: Labels) -> Hand {
        let hand = Hand {
            bid,
            cards,
            labels,
            rank: HighCard,
        };

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
        let count = counts.get(0).map(|(_, count)| *count).unwrap_or(0);
        let second_count = counts.get(1).map(|(_, count)| *count).unwrap_or(0);
        match_counts(count, second_count)
    }

    fn get_rank_with_joker(&mut self) -> Rank {
        let (counts, joker_count) = get_card_counts(&self.cards);
        let mut counts: Vec<(char, u8)> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let count = counts.get(0).map(|(_, count)| *count).unwrap_or(0);
        let second_count = counts.get(1).map(|(_, count)| *count).unwrap_or(0);
        let mut rank = HighCard;
        match count + joker_count {
            5 => rank = Rank::FiveOfAKind,
            4 => rank = Rank::FourOfAKind,
            3 => {
                if second_count == 2 {
                    rank = Rank::FullHouse
                } else {
                    rank = Rank::ThreeOfAKind
                }
            }
            2 => {
                if second_count == 2 {
                    rank = Rank::TwoPairs
                } else {
                    rank = Rank::OnePair
                }
            }
            _ => (),
        }

        rank
    }

    fn cmp_with_joker(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    if CARD_LABELS_WITH_JOCKER_MAP[&self.cards[i]]
                        > CARD_LABELS_WITH_JOCKER_MAP[&other.cards[i]]
                    {
                        return Ordering::Greater;
                    } else if CARD_LABELS_WITH_JOCKER_MAP[&self.cards[i]]
                        < CARD_LABELS_WITH_JOCKER_MAP[&other.cards[i]]
                    {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
            _ => self.rank.cmp(&other.rank),
        }
    }

    fn cmp_without_joker(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    if CARD_LABELS_MAP[&self.cards[i]] > CARD_LABELS_MAP[&other.cards[i]] {
                        return Ordering::Greater;
                    } else if CARD_LABELS_MAP[&self.cards[i]] < CARD_LABELS_MAP[&other.cards[i]] {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
            _ => self.rank.cmp(&other.rank),
        }
    }
}

fn match_counts(count: u8, second_count: u8) -> Rank {
    let mut rank = HighCard;
    match count {
        5 => rank = Rank::FiveOfAKind,
        4 => rank = Rank::FourOfAKind,
        3 => {
            if second_count == 2 {
                rank = Rank::FullHouse
            } else {
                rank = Rank::ThreeOfAKind
            }
        }
        2 => {
            if second_count == 2 {
                rank = Rank::TwoPairs
            } else {
                rank = Rank::OnePair
            }
        }
        _ => (),
    }
    rank
}

fn get_card_counts(cards: &Vec<char>) -> (HashMap<char, u8>, u8) {
    let mut counts: HashMap<char, u8> = HashMap::new();
    let mut joker_count = 0;

    for card in cards {
        if *card == 'J' {
            joker_count += 1;
            continue;
        } else {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }
    }
    (counts, joker_count)
}

fn hand_from_string(line: &String, labels: Labels) -> Hand {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    let cards: Vec<char> = split_line[0].chars().collect();
    let bid: u32 = split_line[1].parse().unwrap();
    Hand::new(cards, bid, labels)
}

fn build_hand(line: &String) -> Hand {
    let mut hand = hand_from_string(line, Labels::Normal);
    hand.rank = hand.get_rank();

    hand
}

fn build_hand_with_joker(line: &String) -> Hand {
    let mut hand = hand_from_string(line, Labels::WithJoker);
    hand.rank = hand.get_rank_with_joker();

    hand
}

fn hands_from_input(filename: &str, build: HandBuilder) -> Vec<Hand> {
    advent_of_code_2023::read_lines(filename)
        .unwrap()
        .iter()
        .map(build)
        .collect()
}

fn find_total_winnings(filename: &str, build: HandBuilder) -> u64 {
    let mut total = 0;
    let mut hands = hands_from_input(filename, build);
    hands.sort();
    // iterate with index to get the highest ranked hand
    for (i, hand) in hands.iter().enumerate() {
        total += (hand.bid * (i + 1) as u32) as u64;
    }
    total
}

fn main() {
    println!(
        "Part 1: {}",
        find_total_winnings("src/bin/day07/input.txt", build_hand)
    );
    println!(
        "Part 2: {}",
        find_total_winnings("src/bin/day07/input.txt", build_hand_with_joker)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_new() {
        let hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0, Labels::Normal);
        assert_eq!(hand.cards, vec!['2', '3', '4', '5', '6']);
        assert_eq!(hand.rank, Rank::HighCard);
    }

    #[test]
    fn test_hand_get_rank() {
        let mut hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::HighCard);
        hand = Hand::new(vec!['2', '2', '2', '2', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::FourOfAKind);
        hand = Hand::new(vec!['2', '2', '2', '6', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::FullHouse);
        hand = Hand::new(vec!['2', '2', '2', '3', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::ThreeOfAKind);
        hand = Hand::new(vec!['2', '2', '3', '3', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::TwoPairs);
        hand = Hand::new(vec!['2', '2', '3', '4', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::OnePair);
        hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0, Labels::Normal);
        assert_eq!(hand.get_rank(), Rank::HighCard);
    }

    #[test]
    fn test_hand_cmp() {
        let mut hands = hands_from_input("src/bin/day07/test_input.txt", build_hand);
        hands.sort();
        assert_eq!(hands[0].cards, vec!['3', '2', 'T', '3', 'K']);
        assert_eq!(hands[1].cards, vec!['K', 'T', 'J', 'J', 'T']);
        assert_eq!(hands[2].cards, vec!['K', 'K', '6', '7', '7']);
        assert_eq!(hands[3].cards, vec!['T', '5', '5', 'J', '5']);
        assert_eq!(hands[4].cards, vec!['Q', 'Q', 'Q', 'J', 'A']);
    }

    #[test]
    fn test_hands_from_input() {
        let hands = hands_from_input("src/bin/day07/test_input.txt", build_hand);
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
        assert_eq!(
            find_total_winnings("src/bin/day07/test_input.txt", build_hand),
            6440
        );
    }

    #[test]
    fn test_hand_get_rank_with_joker() {
        let mut hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::HighCard);
        hand = Hand::new(vec!['2', '2', '2', '2', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FourOfAKind);
        hand = Hand::new(vec!['2', '2', '2', '6', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FullHouse);
        hand = Hand::new(vec!['2', '2', '2', '3', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::ThreeOfAKind);
        hand = Hand::new(vec!['2', '2', '3', '3', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::TwoPairs);
        hand = Hand::new(vec!['2', '2', '3', '4', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::OnePair);
        hand = Hand::new(vec!['2', '3', '4', '5', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::HighCard);
        hand = Hand::new(vec!['J', '2', '3', '4', '5'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::OnePair);
        hand = Hand::new(vec!['J', '2', '2', '2', '2'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FiveOfAKind);
        hand = Hand::new(vec!['J', '2', '2', '2', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FourOfAKind);
        hand = Hand::new(vec!['J', '2', '2', '6', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FullHouse);
        hand = Hand::new(vec!['J', '2', '2', '3', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::ThreeOfAKind);
        hand = Hand::new(vec!['J', '2', '3', '3', '6'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::ThreeOfAKind);
        hand = Hand::new(vec!['T', '5', '5', 'J', '5'], 0, Labels::WithJoker);
        assert_eq!(hand.get_rank_with_joker(), Rank::FourOfAKind);
    }

    #[test]
    fn test_hands_from_input_with_joker() {
        let hands = hands_from_input("src/bin/day07/test_input.txt", build_hand_with_joker);
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
        assert_eq!(hands[1].rank, Rank::FourOfAKind);
        assert_eq!(hands[2].rank, Rank::TwoPairs);
        assert_eq!(hands[3].rank, Rank::FourOfAKind);
        assert_eq!(hands[4].rank, Rank::FourOfAKind);
    }

    #[test]
    fn test_hands_with_joker_cmp() {
        let mut hands = hands_from_input("src/bin/day07/test_input.txt", build_hand_with_joker);
        hands.sort();
        assert_eq!(hands[0].cards, vec!['3', '2', 'T', '3', 'K']);
        assert_eq!(hands[1].cards, vec!['K', 'K', '6', '7', '7']);
        assert_eq!(hands[2].cards, vec!['T', '5', '5', 'J', '5']);
        assert_eq!(hands[3].cards, vec!['Q', 'Q', 'Q', 'J', 'A']);
        assert_eq!(hands[4].cards, vec!['K', 'T', 'J', 'J', 'T']);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            find_total_winnings("src/bin/day07/test_input.txt", build_hand_with_joker),
            5905
        );
    }
}
