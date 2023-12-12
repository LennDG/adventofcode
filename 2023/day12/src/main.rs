use color_eyre::eyre::Ok;

mod parser;

fn main() {
    let input = include_str!("example");
    let condition_reports = parser::parse_condition_reports(input);
    part_one(condition_reports)
}

fn part_one(condition_reports: Vec<ConditionReport>) {}

// Checks up to the first Unknown if the conditions match the groups
fn check_finished(report: &ConditionReport) -> bool {
    let finished: Vec<Condition> = report
        .conditions
        .iter()
        .take_while(|c| **c != Condition::Unknown)
        .cloned()
        .collect();

    false
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
