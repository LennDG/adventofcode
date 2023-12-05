use std::collections::{HashMap, HashSet};

mod parser;

fn main() {
    let input = include_str!("input");
    let engine = parser::parse_engine(input);
    //part_one(engine.clone());
    part_two(engine);
}

fn part_one(engine: Engine) {
    let mut sum = 0;
    for (i, engine_line) in engine.schematics.iter().enumerate() {
        let mut integer = Integer::new();
        for (j, schematic_data) in engine_line.iter().enumerate() {
            match schematic_data {
                SchematicData::Digit(value) => {
                    integer.add_digit(*value);
                    if !integer.adjacent {
                        check_adjacent(&mut integer, &engine, i, j)
                    }
                }
                SchematicData::Symbol(_) => {
                    sum += integer.get_value();
                    integer.flush()
                }
                SchematicData::Gear => {
                    sum += integer.get_value();
                    integer.flush()
                }
                SchematicData::Nothing => {
                    sum += integer.get_value();
                    integer.flush()
                }
            }
        }
    }

    println!("The sum of all parts is: {sum}");
}

fn part_two(engine: Engine) {
    let mut integer = Integer::new();
    for (i, engine_line) in engine.schematics.iter().enumerate() {
        for (j, schematic_data) in engine_line.iter().enumerate() {
            match schematic_data {
                SchematicData::Digit(value) => {
                    integer.add_digit(*value);
                    if !integer.adjacent {
                        find_gears(&mut integer, &engine, i, j)
                    }
                }
                SchematicData::Symbol(_) => integer.flush(),
                SchematicData::Gear => integer.flush(),
                SchematicData::Nothing => integer.flush(),
            }
        }
    }

    let mut sum = 0;
    for adj in integer.gears.values() {
        if adj.len() == 2 {
            sum += adj[0] * adj[1];
        }
    }

    println!("The sum of all gear ratios is: {sum}");
}

fn find_gears(integer: &mut Integer, engine: &Engine, i: usize, j: usize) {
    println!("Center: {i}, {j}");

    for y in i - 1..=i + 1 {
        for x in j - 1..=j + 1 {
            println!("Checked: {y}, {x}");
            if let SchematicData::Gear = &engine.schematics[y][x] {
                integer.add_adjacent_gear(y, x)
            }
        }
    }
}

fn check_adjacent(integer: &mut Integer, engine: &Engine, i: usize, j: usize) {
    let neighbours = vec![
        &engine.schematics[i][j - 1],
        &engine.schematics[i][j + 1],
        &engine.schematics[i - 1][j],
        &engine.schematics[i + 1][j],
        &engine.schematics[i - 1][j - 1],
        &engine.schematics[i - 1][j + 1],
        &engine.schematics[i + 1][j - 1],
        &engine.schematics[i + 1][j + 1],
    ];

    for neighbour in neighbours {
        if let SchematicData::Symbol(_) = neighbour {
            integer.adjacent = true;
            return;
        }
    }
}

struct Integer {
    buffer: Vec<u32>,
    adjacent: bool,
    adjacent_gears: HashSet<usize>,
    gears: HashMap<usize, Vec<u32>>,
}

impl Integer {
    fn new() -> Self {
        Integer {
            buffer: vec![],
            adjacent: false,
            adjacent_gears: HashSet::new(),
            gears: HashMap::new(),
        }
    }

    fn add_digit(&mut self, digit: u32) {
        self.buffer.push(digit)
    }

    fn get_value(&self) -> u32 {
        let mut value = 0;
        for (i, b) in self.buffer.iter().rev().enumerate() {
            let v = *b;
            let power = u32::try_from(i).unwrap();

            value += v * 10_u32.pow(power);
        }
        value
    }

    fn add_adjacent_gear(&mut self, i: usize, j: usize) {
        let key = format!("{i}{j}").parse::<usize>().unwrap();
        self.adjacent_gears.insert(key);

        println!("{:?}", self.adjacent_gears)
    }

    fn flush(&mut self) {
        self.flush_to_gears();

        self.buffer = vec![];
        self.adjacent = false;
        self.adjacent_gears = HashSet::new();
    }

    fn flush_to_gears(&mut self) {
        let v = self.get_value();

        for key in &self.adjacent_gears {
            match self.gears.get(key) {
                Some(entry) => {
                    let mut new_entry = entry.clone();
                    new_entry.push(v);
                    self.gears.insert(*key, new_entry);
                }
                None => {
                    self.gears.insert(*key, vec![v]);
                }
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct Engine {
    schematics: Vec<Vec<SchematicData>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SchematicData {
    Digit(u32),
    Symbol(String),
    Gear,
    Nothing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value() {
        let int = Integer {
            buffer: vec![4, 5, 8],
            adjacent: false,
            gears: HashMap::new(),
            adjacent_gears: HashSet::new(),
        };
        assert_eq!(int.get_value(), 458);
    }
}
