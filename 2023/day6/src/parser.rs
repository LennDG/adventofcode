use crate::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

pub fn parse_race(input: &str) -> (u64, u64) {
    let (_, (_, _, time, _, _, distance)) = tuple((
        tag("Time:"),
        multispace1,
        parse_number,
        multispace1,
        tag("Distance:"),
        parse_number,
    ))(input)
    .unwrap();

    (time, distance)
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, u64::from_str)(input.trim())
}
pub fn parse_races(input: &str) -> Vec<Race> {
    let (_, (_, _, times, _, _, distances)) = tuple((
        tag("Time:"),
        multispace1,
        parse_numbers,
        multispace1,
        tag("Distance:"),
        parse_numbers,
    ))(input)
    .unwrap();

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, map_res(digit1, u32::from_str))(input.trim())
}
