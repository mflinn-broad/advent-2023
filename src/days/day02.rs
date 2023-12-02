use std::error::Error;

use crate::util;

const MAX_PULL_RED: usize = 12;
const MAX_PULL_GREEN: usize = 13;
const MAX_PULL_BLUE: usize = 14;

pub fn run() -> Result<(), Box<dyn Error>> {
    util::read_input("inputs/day02.txt")
        .map(|raw_input| {
            let game_list = Game::new_from_list(&raw_input);
            println!("part 1: {}", part_1(&game_list));
            println!("part 2: {}", part_2(&game_list));
        })
        .map_err(|e| e.into())
}

fn part_1(games: &[Game]) -> usize {
    games
        .iter()
        .filter(|&game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

fn part_2(games: &[Game]) -> usize {
    games.iter().map(Game::power).sum()
}

#[derive(Debug)]
struct Pull(usize, Color);

impl From<&str> for Pull {
    fn from(value: &str) -> Self {
        let value = value.trim();
        value
            .split_once(' ')
            .map(|(count, color)| Pull(count.parse().unwrap(), color.into()))
            .unwrap()
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => unimplemented!("encountered invalid pull color"),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    pulls: Vec<Vec<Pull>>,
}

impl Game {
    fn new_from_list(games: &str) -> Vec<Game> {
        games.lines().map(Self::from).collect()
    }

    fn is_possible(&self) -> bool {
        for pull_set in self.pulls.iter() {
            for pull in pull_set {
                match pull.1 {
                    Color::Red if pull.0 > MAX_PULL_RED => return false,
                    Color::Green if pull.0 > MAX_PULL_GREEN => return false,
                    Color::Blue if pull.0 > MAX_PULL_BLUE => return false,
                    _ => continue,
                }
            }
        }
        true
    }

    fn power(&self) -> usize {
        let mut r_max = 0;
        let mut g_max = 0;
        let mut b_max = 0;
        self.pulls.iter().flatten().for_each(|pull| match pull.1 {
            Color::Red if pull.0 > r_max => r_max = pull.0,
            Color::Blue if pull.0 > b_max => b_max = pull.0,
            Color::Green if pull.0 > g_max => g_max = pull.0,
            _ => (),
        });
        r_max * g_max * b_max
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (id, pulls) = value.split_once(": ").unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let pulls: Vec<Vec<Pull>> = pulls
            .split("; ")
            .map(|pulls| pulls.split(", ").map(Pull::from).collect())
            .collect();

        Game {
            id: id.parse().unwrap(),
            pulls,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day02.txt").unwrap();
        b.iter(|| part_1(&Game::new_from_list(&input)));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day02.txt").unwrap();
        b.iter(|| part_2(&Game::new_from_list(&input)));
    }
}
