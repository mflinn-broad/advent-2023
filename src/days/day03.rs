use std::{collections::HashMap, error::Error};

use crate::util;

type Position = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Component {
    PartNumber(usize),
    Symbol(char),
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day03.txt")?;
    let map = to_position_map(process(&raw_input));
    println!("part 1: {}", part_1(&map));
    println!("part 2: {}", part_2(&map));
    Ok(())
}

fn part_1(part_map: &HashMap<(Position, Position), Component>) -> usize {
    let mut sum = 0;
    for (k, v) in part_map.iter() {
        if let Component::PartNumber(pn) = v {
            if adjacent_to_symbol(*k, part_map) {
                sum += pn;
            }
        }
    }
    sum
}

fn part_2(part_map: &HashMap<(Position, Position), Component>) -> usize {
    let mut result = 0;
    for (k, v) in part_map.iter() {
        if let Component::Symbol(sym) = v {
            if *sym == '*' {
                let adjacent_part_nums = find_adjacent_part_nums(*k, part_map);
                if adjacent_part_nums.len() == 2 {
                    let first = match adjacent_part_nums[0] {
                        Component::PartNumber(n) => n,
                        _ => unreachable!(),
                    };
                    let second = match adjacent_part_nums[1] {
                        Component::PartNumber(n) => n,
                        _ => unreachable!(),
                    };
                    result += first * second;
                }
            }
        }
    }

    result
}

fn process(input: &str) -> Vec<Vec<Option<char>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect()
}

fn to_position_map(input: Vec<Vec<Option<char>>>) -> HashMap<(Position, Position), Component> {
    let mut result: HashMap<(Position, Position), Component> = HashMap::new();

    for (row_num, row_data) in input.iter().enumerate() {
        let mut col = 0;
        while col < row_data.len() {
            if row_data[col].is_none() {
                col += 1;
                continue;
            }
            if let Some(v) = row_data[col] {
                match v {
                    v if v.is_ascii_digit() => {
                        let mut num_string = String::new();
                        let start_pos = (row_num, col);
                        // num_string.push(v);
                        while col < row_data.len() && row_data[col].is_some() {
                            if let Some(v) = row_data[col] {
                                if v.is_ascii_punctuation() {
                                    break;
                                }
                            }
                            if let Some(v) = row_data[col] {
                                num_string.push(v);
                                col += 1;
                            }
                        }
                        let num = num_string.parse().unwrap();
                        _ = *result
                            .entry((
                                (start_pos.0 as isize, start_pos.1 as isize),
                                (row_num as isize, (col - 1) as isize),
                            ))
                            .or_insert(Component::PartNumber(num));
                    }
                    v if v.is_ascii_punctuation() => {
                        let pos = (row_num, col);
                        _ = *result
                            .entry((
                                (pos.0 as isize, pos.1 as isize),
                                (pos.0 as isize, pos.1 as isize),
                            ))
                            .or_insert(Component::Symbol(v));
                        col += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    result
}

fn adjacent_to_symbol(
    pos_range: (Position, Position),
    map: &HashMap<(Position, Position), Component>,
) -> bool {
    let (start, end) = pos_range;
    for row in (start.0 - 1)..=(start.0 + 1) {
        for col in (start.1 - 1)..=(end.1 + 1) {
            if let Some(Component::Symbol(_)) = map.get(&((row, col), (row, col))) {
                return true;
            }
        }
    }
    false
}

fn find_adjacent_part_nums(
    pos: (Position, Position),
    part_map: &HashMap<(Position, Position), Component>,
) -> Vec<Component> {
    let mut adjacent_components = Vec::new();
    for (k, v) in part_map.iter() {
        if let Component::Symbol(_) = v {
            continue;
        }
        let (part_num_start, part_num_end) = k;
        let in_range_row = (part_num_start.0 >= pos.0 .0 - 1) && (part_num_start.0 <= pos.0 .0 + 1);
        let in_range_col = (pos.0 .1 >= part_num_start.1 - 1) && (pos.0 .1 <= part_num_end.1 + 1);
        if in_range_row && in_range_col {
            adjacent_components.push(*v);
        }
    }

    adjacent_components
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let p_map = to_position_map(process(&input));
        assert_eq!(part_1(&p_map), 4361);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day03.txt").unwrap();
        b.iter(|| part_1(&to_position_map(process(&input))));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day03.txt").unwrap();
        b.iter(|| part_2(&to_position_map(process(&input))));
    }
}
