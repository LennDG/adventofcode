use color_eyre::eyre::Ok;

mod parser;

fn main() {
    let input = include_str!("test_check_cont");
    let condition_reports = parser::parse_condition_reports(input);
    part_one(condition_reports)
}

fn part_one(condition_reports: Vec<ConditionReport>) {}

// Checks up to the first Unknown if the conditions match the groups
fn check_contiguous_groups(conditions: &[Condition], checked_groups: Vec<u64>) -> bool {
    let finished: Vec<Condition> = conditions
        .iter()
        .take_while(|c| **c != Condition::Unknown)
        .cloned()
        .collect();

    let (mut groups, last): (Vec<u64>, u64) = finished.iter().fold(
        (Vec::new(), 0),
        |(mut groups, mut current_group), condition| {
            match condition {
                Condition::Operational => {
                    groups.push(current_group);
                    current_group = 0;
                }
                Condition::Damaged => current_group += 1,
                Condition::Unknown => (),
            }

            (groups, current_group)
        },
    );
    if last > 0 {
        groups.push(last)
    }

    groups.len() >= checked_groups.len()
        && groups
            .iter()
            .zip(checked_groups.iter())
            .all(|(a, b)| a == b)
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
