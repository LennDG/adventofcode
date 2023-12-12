use crate::{Condition, ConditionReport};
use nom::{
    self,
    character::complete::{anychar, char, digit1, multispace1, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub fn parse_condition_reports(input: &str) -> Vec<ConditionReport> {
    let (_, conditions) = separated_list1(newline, parse_condition_report)(input).unwrap();
    conditions
}

fn parse_condition_report(input: &str) -> IResult<&str, ConditionReport> {
    let (remaining, (conditions, groups)) = separated_pair(
        parse_conditions,
        multispace1,
        separated_list1(char(','), map_res(digit1, u64::from_str)),
    )(input)?;
    Ok((remaining, ConditionReport { conditions, groups }))
}

fn parse_conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    many1(map_res(anychar, Condition::try_from))(input)
}
