use std::collections::HashSet;

use crate::util::read_input;
use anyhow::Result;

pub fn run() -> Result<()> {
    let raw_input = read_input("inputs/day11.txt")?;
    let mut universe: Universe = raw_input.as_str().into();
    universe.expand(2);
    let universe = universe;
    let galaxy_coords = universe.get_galaxy_coords();
    println!("Part 1: {}", galaxy_distances(galaxy_coords));

    let mut universe: Universe = raw_input.as_str().into();
    universe.expand(1000000);
    println!("Part 2: {}", galaxy_distances(universe.get_galaxy_coords()));
    Ok(())
}

fn galaxy_distances(coords: &HashSet<(usize, usize)>) -> usize {
    let mut processed: HashSet<(usize, usize)> = HashSet::new();
    let mut distance_sum = 0;
    for galaxy_1 in coords.iter() {
        for galaxy_2 in coords.iter() {
            if galaxy_1 != galaxy_2 && !processed.contains(galaxy_2) {
                distance_sum += cartesian_distance(*galaxy_1, *galaxy_2);
            }
        }
        processed.insert(*galaxy_1);
    }

    distance_sum
}

#[derive(Debug)]
struct Universe {
    galaxies: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let height = value.lines().count();
        let width = value.lines().next().unwrap().chars().count();
        let galaxies = value
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(
                    move |(col, c)| {
                        if c == '#' {
                            Some((row, col))
                        } else {
                            None
                        }
                    },
                )
            })
            .flatten()
            .collect();

        Universe {
            galaxies,
            width,
            height,
        }
    }
}

impl Universe {
    fn expand_rows(&mut self, factor: usize) {
        let mut i = 0;
        while i < self.height {
            // determine if there are any galaxies on this row, if so
            // increment to next row and continue
            if self.galaxies.iter().any(|(row, _)| *row == i) {
                i += 1;
            } else {
                // increment the row value of all galaxies with .0 == i by factor
                let mut updated_galaxies = HashSet::new();

                self.galaxies.clone().iter().for_each(|(row, col)| {
                    if *row > i {
                        self.galaxies.remove(&(*row, *col));
                        updated_galaxies.insert((*row + (factor - 1), *col));
                    }
                });
                self.galaxies = self
                    .galaxies
                    .clone()
                    .into_iter()
                    .chain(updated_galaxies.into_iter())
                    .collect();
                i += factor;
                self.height += factor - 1;
            }
        }
    }

    fn expand_cols(&mut self, factor: usize) {
        let mut i = 0;
        while i < self.width {
            // determine if there are any galaxies on this row, if so
            // increment to next row and continue
            if self.galaxies.iter().any(|(_, col)| *col == i) {
                i += 1;
            } else {
                // increment the row value of all galaxies with .x == i by factor
                let mut updated_galaxies = HashSet::new();
                self.galaxies.clone().iter().for_each(|(row, col)| {
                    if *col > i {
                        self.galaxies.remove(&(*row, *col));
                        updated_galaxies.insert((*row, *col + (factor - 1)));
                    }
                });
                self.galaxies = self
                    .galaxies
                    .clone()
                    .into_iter()
                    .chain(updated_galaxies.into_iter())
                    .collect();
                i += factor;
                self.width += factor - 1;
            }
        }
    }

    fn expand(&mut self, factor: usize) {
        self.expand_rows(factor);
        self.expand_cols(factor);
    }

    fn get_galaxy_coords(&self) -> &HashSet<(usize, usize)> {
        &self.galaxies
    }
}

fn cartesian_distance(coord: (usize, usize), other: (usize, usize)) -> usize {
    coord.0.abs_diff(other.0) + coord.1.abs_diff(other.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn basic_test() {
        let universe = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut universe: Universe = universe.into();
        universe.expand(2);
        assert_eq!(374, galaxy_distances(universe.get_galaxy_coords()));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let mut universe: Universe = raw_input.as_str().into();
            universe.expand(2);
            galaxy_distances(universe.get_galaxy_coords());
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let raw_input = read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let mut universe: Universe = raw_input.as_str().into();
            universe.expand(1000000);
            galaxy_distances(universe.get_galaxy_coords());
        })
    }
}
