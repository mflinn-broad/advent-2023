use std::error::Error;

use crate::util;

pub fn run() -> Result<(), Box<dyn Error>> {
    util::read_input("inputs/day01.txt").and_then(|raw_input| {
        Ok((part_1(&raw_input), part_1(&transform(&raw_input))))
    })
    .and_then(|(p1, p2)| {
        println!("part 1: {}", p1);
        println!("part 2: {}", p2);
        Ok(())
    })
    .map_err(|e| e.into())
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let nums: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let mut filtered_num = String::new();
            filtered_num.push(nums.chars().next().unwrap());
            filtered_num.push(nums.chars().next_back().unwrap());

            filtered_num.parse::<u64>().unwrap()
        })
        .sum()
}

#[derive(Debug)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<&str> for Digit {
    fn from(value: &str) -> Self {
        match value {
            "one" => Self::One,
            "two" => Self::Two,
            "three" => Self::Three,
            "four" => Self::Four,
            "five" => Self::Five,
            "six" => Self::Six,
            "seven" => Self::Seven,
            "eight" => Self::Eight,
            "nine" => Self::Nine,
            _ => unreachable!(),
        }
    }
}

impl From<Digit> for char {
    fn from(val: Digit) -> Self {
        match val {
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        }
    }
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn transform(input: &str) -> String {
    input
        .lines()
        .map(|mut line| {
            let mut transformed = String::new();
            while !line.is_empty() {
                if line.starts_with(|c: char| c.is_ascii_digit()) {
                    transformed.push(line.chars().next().unwrap());
                    line = &line[1..];
                    continue;
                }
                for alpha_digit in DIGITS {
                    if line.starts_with(alpha_digit) {
                        transformed.push(Digit::from(alpha_digit).into());
                        line = &line[alpha_digit.len() - 2..];
                    }
                }
                line = &line[1..];
            }
            transformed.push('\n');
            transformed
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day01.txt").unwrap();
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day01.txt").unwrap();
        b.iter(|| part_1(&transform(&input)));
    }
}
