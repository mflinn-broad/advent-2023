use crate::util;
use itertools::Itertools;
use rangemap::RangeMap;
use std::{error::Error, ops::Range};

type Seeds = Vec<Range<i64>>;
type SeedMaps = Vec<RangeMap<i64, i64>>;

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day05.txt")?;
    let (seeds, seed_maps) = process(&raw_input);
    println!("part 1: {}", part_1(seeds.clone(), seed_maps.clone()));
    Ok(())
}

fn part_1(mut seeds: Seeds, seed_maps: SeedMaps) -> i64 {
    for map in seed_maps {
        seeds = apply_map(&mut seeds, &map);
    }
    seeds.iter().map(|range| range.start).min().unwrap()
}

fn process(input: &str) -> (Seeds, SeedMaps) {
    let mut blocks = input.split("\n\n");

    let (_, seeds) = blocks.next().unwrap().split_once(": ").unwrap();
    let seeds: Vec<Range<i64>> = seeds
        .split_ascii_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start: i64 = chunk.next().unwrap().parse().unwrap();
            let length: i64 = chunk.next().unwrap().parse().unwrap();
            start..start + length
        })
        .collect();
    let mut seed_maps = Vec::new();
    for block in blocks {
        let mut seed_map: RangeMap<i64, i64> = RangeMap::new();
        for line in block.lines().skip(1) {
            let mut components = line.split_ascii_whitespace();
            let dst: i64 = components.next().unwrap().parse().unwrap();
            let src: i64 = components.next().unwrap().parse().unwrap();
            let length: i64 = components.next().unwrap().parse().unwrap();

            seed_map.insert(src..src + length, dst - src);
        }
        seed_maps.push(seed_map);
    }

    (seeds, seed_maps)
}

fn apply_map(inputs: &mut Vec<Range<i64>>, map: &RangeMap<i64, i64>) -> Vec<Range<i64>> {
    let mut out = Vec::new();
    while let Some(input) = inputs.pop() {
        if map.overlaps(&input) {
            for (range, offset) in map.overlapping(&input) {
                let start = std::cmp::max(input.start, range.start);
                let end = std::cmp::min(input.end, range.end);
                out.push(start + offset..end + offset);
                if input.start < start {
                    inputs.push(input.start..start);
                }
                if end < input.end {
                    inputs.push(end..input.end);
                }
            }
        } else {
            out.push(input);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day05.txt").unwrap();
        let (seeds, seed_map) = process(&input);
        b.iter(|| part_1(seeds.clone(), seed_map.clone()));
    }
}
