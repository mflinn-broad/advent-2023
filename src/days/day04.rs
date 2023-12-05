use std::error::Error;
use std::collections::HashSet;

use crate::util;

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day04.txt")?;
    let cards = Card::from_card_list(&raw_input);
    println!("part 1: {}", part_1(&cards));
    println!("part 2: {}", part_2(&cards));

    Ok(())
}

fn part_1(cards: &[Card]) -> usize {
    cards.iter()
        .map(Card::score)
        .sum()
}

fn part_2(cards: &[Card]) -> usize {
    let mut result = vec![1; cards.len()];
    cards.iter()
        .enumerate()
        .for_each(|(idx, card)| {
            let winner_count = card.winning_numbers().len();
            for i in (idx+1)..=(idx + winner_count) {
                result[i] += result[idx]
            }
        });

    result.iter().sum()
}

struct Card {
    winning_numbers: HashSet<usize>,
    my_numbers: HashSet<usize>,
}

impl Card {
    fn winning_numbers(&self) -> Vec<usize> {
        self.winning_numbers.intersection(&self.my_numbers).cloned().collect()
    }

    fn from_card_list(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(Card::from)
            .collect()
    }

    fn score(&self) -> usize {
        if self.winning_numbers().is_empty() {
            return 0;
        }
        self.winning_numbers()
            .iter()
            .skip(1)
            .fold(1, |score, _| score * 2)
    }
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let (_, card_data) = s.split_once(": ").unwrap();
        let (winners, mine) = card_data.split_once(" | ").unwrap();
        let winners = winners.trim();
        let mine = mine.trim();
        let winning_numbers = winners.split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let my_numbers = mine.split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        Self { winning_numbers, my_numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day04.txt").unwrap();
        b.iter(|| part_1(&Card::from_card_list(&input)));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day04.txt").unwrap();
        b.iter(|| part_2(&Card::from_card_list(&input)));
    }
}
