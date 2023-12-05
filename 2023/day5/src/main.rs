use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

mod parser;

fn main() {
    let input = include_str!("example");
    let almanac = parser::parse_almanac(input);
    //println!("{:?}", almanac);
    part_one(almanac.clone());
    part_two(almanac)
}

fn part_one(almanac: Almanac) {
    let min_location = almanac
        .seeds
        .iter()
        .map(|seed| almanac.map(*seed))
        .min()
        .unwrap();
    println!("Minimum location is {min_location}")
}

fn part_two(almanac: Almanac) {
    let seed_ranges = almanac.get_seed_ranges();
    let min_location = seed_ranges
        .iter()
        .map(|(start, length)| {
            (*start..=*start + *length)
                .into_par_iter()
                .map(|seed| almanac.map(seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("Minimum location is {min_location}")
}

fn reverse_brute_force_part_two(almanac: Almanac) {
    let seed_ranges = almanac.get_seed_ranges();
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn map(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |acc, map| map.map(acc))
    }

    fn get_seed_ranges(&self) -> Vec<(u64, u64)> {
        let mut seed_ranges: Vec<(u64, u64)> = vec![];

        for i in (0..self.seeds.len()).step_by(2) {
            if let Some(range) = self.seeds.get(i + 1) {
                seed_ranges.push((self.seeds[i], *range));
            }
        }

        seed_ranges
    }
}

#[derive(Debug, Clone)]
pub struct MapLine {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug, Clone)]
pub struct Map {
    lines: Vec<MapLine>,
    // from: Source,
    // to: Source,
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        match self
            .lines
            .iter()
            .filter(|line| {
                value >= line.source_range_start
                    && value <= line.source_range_start + line.range_length
            })
            .map(|line| line.destination_range_start + (value - line.source_range_start))
            .next()
        {
            Some(mapped) => mapped,
            None => value,
        }
    }

    fn reverse_map(&self, value: u64) -> u64 {
        match self
            .lines
            .iter()
            .filter(|line| {
                value >= line.destination_range_start
                    && value <= line.destination_range_start + line.range_length
            })
            .map(|line| line.source_range_start + (value - line.destination_range_start))
            .next()
        {
            Some(mapped) => mapped,
            None => value,
        }
    }
}

pub enum Source {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map {
            // from: Source::Seed,
            // to: Source::Soil,
            lines: vec![
                MapLine {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                MapLine {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48,
                },
            ],
        };

        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(62), 64);
        assert_eq!(map.map(10), 10);
    }

    #[test]
    fn test_reverse_map() {
        let map = Map {
            // from: Source::Seed,
            // to: Source::Soil,
            lines: vec![
                MapLine {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2,
                },
                MapLine {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48,
                },
            ],
        };

        assert_eq!(map.reverse_map(50), 98);
        assert_eq!(map.map(52), 50);
        assert_eq!(map.map(64), 62);
        assert_eq!(map.map(10), 10);
    }
}
