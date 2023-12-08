use std::collections::HashMap;

use crate::util;
use anyhow::Result;

pub fn run() -> Result<()> {
    let raw_input = util::read_input("inputs/day08.txt")?;
    let (instructions, node_map) = process(&raw_input);
    println!("part 1: {}", part_1(instructions.chars(), &node_map));
    println!("part 2: {}", part_2(instructions.chars(), &node_map));
    Ok(())
}

fn part_1<I>(instructions: I, node_map: &HashMap<String, (String, String)>) -> usize
where
    I: Iterator<Item = char> + Clone,
{
    let mut num_steps = 0;
    let mut curr_node = String::from("AAA");
    for instruction in instructions.cycle() {
        if curr_node == *"ZZZ" {
            break;
        }
        let (left, right) = node_map.get(&curr_node).unwrap();
        match instruction {
            'L' => curr_node = left.clone().to_owned(),
            'R' => curr_node = right.clone().to_owned(),
            _ => unreachable!(),
        }
        num_steps += 1;
    }

    num_steps
}

fn part_2<I>(instructions: I, node_map: &HashMap<String, (String, String)>) -> usize
where
    I: Iterator<Item = char> + Clone,
{
    node_map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| {
            let mut step_count = 0;
            let mut curr_node = k.to_owned();
            for instruction in instructions.clone().cycle() {
                if curr_node.ends_with('Z') {
                    break;
                }
                let (left, right) = node_map.get(&curr_node).unwrap();
                match instruction {
                    'L' => curr_node = left.clone().to_owned(),
                    'R' => curr_node = right.clone().to_owned(),
                    _ => unreachable!(),
                }
                step_count += 1
            }
            step_count
        })
        .reduce(num_integer::lcm)
        .unwrap()
}

fn process(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (instructions, node_map) = input.split_once("\n\n").unwrap();
    let instructions = String::from(instructions);

    let node_map = node_map
        .lines()
        .map(|line| {
            let (node, next_nodes) = line.split_once(" = ").unwrap();
            let node = String::from(node);
            let next_nodes = next_nodes.replace('(', "");
            let next_nodes = next_nodes.replace(')', "");
            let (left, right) = next_nodes.split_once(", ").unwrap();
            (node, (String::from(left), String::from(right)))
        })
        .collect();

    (instructions, node_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day08.txt").unwrap();
        let (instructions, node_map) = process(&input);
        b.iter(|| part_1(instructions.chars(), &node_map))
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day08.txt").unwrap();
        let (instructions, node_map) = process(&input);
        b.iter(|| part_2(instructions.chars(), &node_map))
    }
}
