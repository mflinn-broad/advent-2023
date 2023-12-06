use std::error::Error;

use crate::util;

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day06.txt")?;
    let races = Race::from_race_list(&raw_input);

    println!("part 1: {}", part_1(&races));

    let p2_race = Race::from_race_list_p2(&raw_input);
    println!("part 2: {}", p2_race.ways_to_win());

    Ok(())
}

fn part_1(races: &[Race]) -> usize {
    races.iter().map(Race::ways_to_win).product()
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: i64,
    distance_record: i64,
}

impl Race {
    fn from_race_list(race_list: &str) -> Vec<Race> {
        let mut lines = race_list.lines();

        let times = lines.next().unwrap();
        let distances = lines.next().unwrap();

        let times = times
            .split_ascii_whitespace()
            .skip(1)
            .map(|time| time.parse::<i64>().unwrap());
        let distances = distances
            .split_ascii_whitespace()
            .skip(1)
            .map(|distance| distance.parse::<i64>().unwrap());

        times
            .zip(distances)
            .map(|(time, distance)| Race {
                time,
                distance_record: distance,
            })
            .collect()
    }

    fn from_race_list_p2(race_list: &str) -> Race {
        let mut lines = race_list.lines();

        let (_, times) = lines.next().unwrap().split_once(':').unwrap();
        let (_, distances) = lines.next().unwrap().split_once(':').unwrap();
        let time: String = times.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        let distance: String = distances
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();

        Race {
            time: time.parse().unwrap(),
            distance_record: distance.parse().unwrap(),
        }
    }

    fn ways_to_win(&self) -> usize {
        (0..=self.time)
            .filter(|charge_time| charge_time * (self.time - charge_time) >= self.distance_record)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 7,
            distance_record: 9,
        };
        assert_eq!(race.ways_to_win(), 4);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day06.txt").unwrap();
        b.iter(|| part_1(&Race::from_race_list(&input)));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day06.txt").unwrap();
        b.iter(|| Race::from_race_list_p2(&input).ways_to_win());
    }
}
