use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day01.txt").unwrap();
    let transformed = transform(&raw_input);
    println!("part 1: {}", part1(&raw_input));
    println!("part 2: {}", part1(&transformed));
}

fn part1(input: &str) -> u64 {
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

impl Into<char> for Digit {
    fn into(self) -> char {
        match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
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
                        continue;
                    }
                }
                line = &line[1..];
            }
            transformed.push('\n');
            transformed
        })
        .collect()
}
