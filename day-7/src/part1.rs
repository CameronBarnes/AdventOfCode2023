use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn score(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

const CARD_ORD: &[&str] = &["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    hand: String,
    kind: HandType,
    bid: usize,
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

impl Hand {
    fn new(hand: String, bid: usize) -> Self {
        let kind = get_hand_type(&hand);
        Hand{hand, kind, bid}
    }
}

fn card_score(c: char) -> usize {
    for i in 0..CARD_ORD.len() {
        if CARD_ORD[i].eq(&c.to_string()) {
            return i;
        }
    }
    panic!("Invalid card entered");
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_ord = self.kind.score().cmp(&other.kind.score());
        if kind_ord.is_eq() {
            for (a, b) in self.hand.chars().zip(other.hand.chars()) {
                let card_ord = card_score(a).cmp(&card_score(b));
                if card_ord.is_ne() {
                    return card_ord.reverse();
                }
            }
            Equal
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

    let mut iter = str.split_whitespace();
    Hand::new(iter.next().unwrap().to_owned(), iter.last().unwrap().parse().unwrap())

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let mut hands: Vec<Hand> = input.lines().map(parse_hand).collect();
    hands.sort();

    let mut result: usize = 0;
    (0..hands.len()).rev().for_each(|i| {
        result += (i + 1) * hands[i].bid;
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
