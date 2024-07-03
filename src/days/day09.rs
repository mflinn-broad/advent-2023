use crate::util;
use anyhow::Result;

pub fn run() -> Result<()> {
    let raw_input = util::read_input("inputs/day09.txt")?;
    let input = process(&raw_input);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
    Ok(())
}

fn part_1(input: Vec<Vec<i32>>) -> i32 {
    input.iter().map(|l| reduce_pattern(l)).sum()
}

fn part_2(mut input: Vec<Vec<i32>>) -> i32 {
    input
        .iter_mut()
        .map(|l| {
            l.reverse();
            reduce_pattern(l)
        })
        .sum()
}

fn process(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn reduce_pattern(input: &[i32]) -> i32 {
    if input.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<i32> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    input[input.len() - 1] + reduce_pattern(&diffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let input = process(test_input.into());
        assert_eq!(part_1(input), 114);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_1(input);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            part_2(input);
        })
    }
}
