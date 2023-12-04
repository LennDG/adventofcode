use super::*;

// This adds an additional border of Nothing around the engine to make using it have less edge cases
pub fn parse_engine(input: &str) -> Engine {
    let mut engine_lines: Vec<Vec<SchematicData>> = vec![];

    let width = input.lines().next().unwrap().len();
    let border = vec![SchematicData::Nothing; width + 2];

    engine_lines.push(border.clone());
    for line in input.lines() {
        let mut engine_line: Vec<SchematicData> = vec![];

        engine_line.push(SchematicData::Nothing);
        for c in line.chars() {
            if c == '.' {
                engine_line.push(SchematicData::Nothing);
            } else if c == '*' {
                engine_line.push(SchematicData::Gear);
            } else if c.is_ascii_digit() {
                engine_line.push(SchematicData::Digit(c.to_digit(10).unwrap()));
            } else {
                engine_line.push(SchematicData::Symbol(c.to_string()))
            }
        }
        engine_line.push(SchematicData::Nothing);

        engine_lines.push(engine_line)
    }
    engine_lines.push(border);

    Engine {
        schematics: engine_lines,
    }
}
