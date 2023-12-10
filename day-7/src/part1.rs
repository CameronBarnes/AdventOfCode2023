use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn get_hand_type(str: &str) -> HandType {

    let mut hash_map: HashMap<char, usize> = HashMap::new();
    str.chars().for_each(|c| {
        hash_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    });

    match hash_map.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => {
            if hash_map.values().contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        },
        2 => {
            if hash_map.values().contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        },
        1 => HandType::FiveOfAKind,
        _ => panic!("Invalid hand")
    }

}

const CARD_ORD: &[&str] = &["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

fn card_score(c: char) -> u8 {
    for i in 0..CARD_ORD.len() {
        if CARD_ORD[i].eq(&c.to_string()) {
            return i as u8;
        }
    }
    panic!("Invalid card entered");
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    hand: (u8, u8, u8, u8, u8),
    kind: HandType,
    bid: u32,
}

impl Hand {
    fn new(hand: &str, bid: u32) -> Self {
        let kind = get_hand_type(hand);
        let hand = parse_cards(hand);
        Hand{hand, kind, bid}
    }
}

fn parse_cards(hand: &str) -> (u8, u8, u8, u8, u8) {
    hand.chars().map(card_score).tuples().next().unwrap()
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_ord = (self.kind as u8).cmp(&(other.kind as u8));
        if kind_ord.is_eq() {
            other.hand.cmp(&self.hand)
        } else {
            kind_ord
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

fn parse_hand(str: &str) -> Hand {

    let (cards, bid) = str.split_once(' ').unwrap();
    Hand::new(cards, bid.parse().unwrap())

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let mut hands: Vec<Hand> = input.lines().map(parse_hand).collect();
    hands.sort();

    let mut result: u32 = 0;
    (0..hands.len()).rev().for_each(|i| {
        result += (i + 1) as u32 * hands[i].bid;
    });

    result.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input));
    }
}
