fn main() {
    println!("Hello, world!");
}

pub struct Seeds {
    seed_numbers: Vec<u32>,
}

pub struct MapLine {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

pub struct Map {
    lines: Vec<MapLine>,
    from: Source,
    to: Source,
}

impl Map {
    fn map(&self, value: u32) -> u32 {
        let mut mapped = value;
        for line in &self.lines {
            if value >= line.source_range_start
                && value <= line.source_range_start + line.range_length
            {
                mapped = line.destination_range_start + (value - line.source_range_start);
                break;
            }
        }
        mapped
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
            from: Source::Seed,
            to: Source::Soil,
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
}
