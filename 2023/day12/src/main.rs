use color_eyre::eyre::Ok;

mod parser;

fn main() {
    let input = include_str!("example");
    let condition_reports = parser::parse_condition_reports(input);
    println!("{condition_reports:?}")
}

#[derive(Debug, Clone)]
pub struct ConditionReport {
    conditions: Vec<Condition>,
    groups: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(color_eyre::eyre::eyre!("not a valid condition: {c:?}")),
        }
    }
}
