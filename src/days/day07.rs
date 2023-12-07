use std::{cmp::Ordering, collections::HashMap, error::Error, fmt::Display, str::FromStr};

use crate::util;
use anyhow::Result;

pub fn run() -> Result<()> {
    let raw_input = util::read_input("inputs/day07.txt")?;
    let hands = Hand::from_hand_list(&raw_input);

    println!("part 2: {}", part_1(hands.clone()));
    Ok(())
}

fn part_1(mut hands: Vec<Hand>) -> usize {
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid() as usize * (i + 1))
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Hand(Card, Card, Card, Card, Card, i64);

impl Hand {
    fn from_hand_list(hands: &str) -> Vec<Hand> {
        hands.lines().flat_map(Hand::from_str).collect()
    }

    fn bid(&self) -> i64 {
        self.5
    }
}

impl FromStr for Hand {
    type Err = CardFromStringErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let c1 = hand[0..1].parse()?;
        let c2 = hand[1..2].parse()?;
        let c3 = hand[2..3].parse()?;
        let c4 = hand[3..4].parse()?;
        let c5 = hand[4..].parse()?;

        Ok(Hand(c1, c2, c3, c4, c5, bid))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
            && self.4 == other.4
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_rank: HandRank = self.into();
        let other_hand_rank: HandRank = other.into();
        if self_hand_rank == other_hand_rank {
            if self.0 != other.0 {
                return self.0.cmp(&other.0);
            } else if self.1 != other.1 {
                return self.1.cmp(&other.1);
            } else if self.2 != other.2 {
                return self.2.cmp(&other.2);
            } else if self.3 != other.3 {
                return self.3.cmp(&other.3);
            } else if self.4 != other.4 {
                return self.4.cmp(&other.4);
            } else {
                return Ordering::Equal;
            }
        }
        self_hand_rank.cmp(&other_hand_rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

impl From<&Hand> for HandRank {
    fn from(value: &Hand) -> Self {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();
        let c1 = card_counts.entry(value.0).or_insert(0);
        *c1 += 1;
        let c2 = card_counts.entry(value.1).or_insert(0);
        *c2 += 1;
        let c3 = card_counts.entry(value.2).or_insert(0);
        *c3 += 1;
        let c4 = card_counts.entry(value.3).or_insert(0);
        *c4 += 1;
        let c5 = card_counts.entry(value.4).or_insert(0);
        *c5 += 1;

        if let Some(joker_count) = card_counts.remove(&Card::J) {
            let most = card_counts
                .iter()
                .max_by(|(_, count), (_, count_b)| count.cmp(count_b))
                .map(|(k, _)| k);

            if let Some(m) = most {
                card_counts.entry(*m).and_modify(|c| *c += joker_count);
            }
        }
        match card_counts.len() {
            1 | 0 => HandRank::FiveOAK,
            2 => {
                for (_card, count) in card_counts.iter() {
                    if *count == 4 {
                        return HandRank::FourOAK;
                    }
                }
                HandRank::FullHouse
            }
            3 => {
                for (_card, count) in card_counts.iter() {
                    if *count == 3 {
                        return HandRank::ThreeOAK;
                    }
                }
                HandRank::TwoPair
            }
            4 => HandRank::Pair,
            5 => HandRank::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    J,
    Tw,
    Thr,
    F,
    Fi,
    S,
    Sv,
    E,
    N,
    T,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = CardFromStringErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::N),
            "8" => Ok(Card::E),
            "7" => Ok(Card::Sv),
            "6" => Ok(Card::S),
            "5" => Ok(Card::Fi),
            "4" => Ok(Card::F),
            "3" => Ok(Card::Thr),
            "2" => Ok(Card::Tw),
            _ => Err(CardFromStringErr {}),
        }
    }
}

#[derive(Debug)]
struct CardFromStringErr {}

impl Display for CardFromStringErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to convert &str to a card")
    }
}

impl Error for CardFromStringErr {}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[test]
    fn test_hand_to_rank() {
        let test_hand = &Hand(Card::A, Card::A, Card::A, Card::E, Card::E, 0);

        let rank: HandRank = test_hand.into();
        assert_eq!(rank, HandRank::FullHouse);

        let test_hand = &Hand(Card::K, Card::K, Card::Sv, Card::S, Card::S, 0);
        let rank: HandRank = test_hand.into();
        assert_eq!(rank, HandRank::TwoPair);
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = util::read_input("inputs/day07.txt").unwrap();
        let hands = Hand::from_hand_list(&input);
        b.iter(|| part_1(hands.clone()));
    }
}
