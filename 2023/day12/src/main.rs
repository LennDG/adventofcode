use std::fmt::Debug;

use color_eyre::eyre::Ok;
use itertools::Itertools;

mod parser;

fn main() {
    let input = include_str!("example");
    let condition_reports = parser::parse_condition_reports(input);
    //part_one(condition_reports.clone());
    part_two(transform_input_part_two(condition_reports))
}

fn part_one(condition_reports: Vec<ConditionReport>) {
    let count = condition_reports.iter().flat_map(brute_force).count();
    println!("total number of solutions: {count}")
}

fn part_two(condition_reports: Vec<ConditionReport>) {
    let count = condition_reports.iter().flat_map(brute_force).count();
    println!("total number of solutions: {count}")
}

fn transform_input_part_two(condition_reports: Vec<ConditionReport>) -> Vec<ConditionReport> {
    condition_reports
        .iter()
        .map(|report| ConditionReport {
            conditions: Itertools::intersperse(
                vec![report.conditions.clone(); 5].iter(),
                &vec![Condition::Unknown],
            )
            .flatten()
            .cloned()
            .collect(),
            groups: vec![report.groups.clone(); 5]
                .iter()
                .flatten()
                .cloned()
                .collect(),
        })
        .collect()
}

fn brute_force(condition_report: &ConditionReport) -> Vec<Vec<Condition>> {
    println!(
        "valid permutations for {:?}, {:?}:",
        condition_report.groups, condition_report.conditions
    );

    let permutations: Vec<Vec<Condition>> = condition_report
        .get_permutations()
        .filter(|conditions| {
            check_groups(&ConditionReport {
                conditions: conditions.clone(),
                groups: condition_report.groups.clone(),
            })
        })
        .collect();

    println!("{:?}\n", permutations);

    permutations
}

fn check_groups(report: &ConditionReport) -> bool {
    let (mut groups, last): (Vec<u64>, u64) = report.conditions.iter().fold(
        (Vec::new(), 0),
        |(mut groups, mut current_group), condition| {
            match condition {
                Condition::Operational => {
                    if current_group > 0 {
                        groups.push(current_group);
                        current_group = 0;
                    }
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

    groups.len() == report.groups.len()
        && groups.iter().zip(report.groups.iter()).all(|(a, b)| a == b)
}

#[derive(Debug, Clone)]
pub struct ConditionReport {
    conditions: Vec<Condition>,
    groups: Vec<u64>,
}

impl ConditionReport {
    fn get_permutations(&self) -> impl Iterator<Item = Vec<Condition>> {
        let unknown_amount = self
            .conditions
            .iter()
            .filter(|c| **c == Condition::Unknown)
            .count();

        let options = [Condition::Damaged, Condition::Operational];
        let init: Vec<Vec<Condition>> = options.iter().map(|a| vec![*a]).collect();
        let combinations = (1..unknown_amount).fold(init, |acc, _| {
            acc.iter()
                .cartesian_product(options.iter())
                .map(|(a, b)| {
                    let mut n = a.clone();
                    n.push(*b);
                    n
                })
                .collect::<Vec<Vec<Condition>>>()
        });

        ConditionPermutationIterator {
            base: self.conditions.clone(),
            combinations: Box::new(combinations.into_iter()),
        }
    }
}

pub struct ConditionPermutationIterator {
    base: Vec<Condition>,
    combinations: Box<dyn Iterator<Item = Vec<Condition>>>,
}

// Iterator for condition report generates permutations
impl Iterator for ConditionPermutationIterator {
    type Item = Vec<Condition>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.combinations.next() {
            Some(permutations) => {
                let mut permutation = permutations.iter();

                Some(
                    self.base
                        .iter()
                        .map(|c| match c {
                            Condition::Operational => Condition::Operational,
                            Condition::Damaged => Condition::Damaged,
                            Condition::Unknown => *permutation.next().unwrap(),
                        })
                        .collect(),
                )
            }
            None => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_group_valid() {
        let input = include_str!("test_check_group_valid");
        let condition_reports = parser::parse_condition_reports(input);

        assert!(condition_reports.iter().all(check_groups))
    }

    #[test]
    fn test_check_group_invalid() {
        let input = include_str!("test_check_group_invalid");
        let condition_reports = parser::parse_condition_reports(input);

        assert!(condition_reports.iter().all(|report| !check_groups(report)))
    }

    #[test]
    fn test_permutations() {
        let input = include_str!("test_permutations");
        let condition_reports = parser::parse_condition_reports(input);

        let permutations: Vec<Vec<Condition>> = condition_reports[0].get_permutations().collect();
        println!("{permutations:?}")
    }
}
