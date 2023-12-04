mod parser;

fn main() {
    let input = include_str!("input");
    let engine = parser::parse_engine(input);
    part_one(engine)
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
                SchematicData::Symbol(_) => sum += integer.flush(),
                SchematicData::Gear => sum += integer.flush(),
                SchematicData::Nothing => sum += integer.flush(),
            }
        }
    }

    println!("The sum of all parts is: {sum}");
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
}

impl Integer {
    fn new() -> Self {
        Integer {
            buffer: vec![],
            adjacent: false,
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

    fn flush(&mut self) -> u32 {
        let mut value = 0;
        if self.adjacent {
            value = self.get_value();
        }
        self.buffer = vec![];
        self.adjacent = false;

        value
    }
}

#[derive(Debug)]
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
        };
        assert_eq!(int.get_value(), 458);
    }
}
