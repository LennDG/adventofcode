use std::str::FromStr;

use nom::{
    character::complete::{digit1, line_ending, newline},
    combinator::{map_res, opt},
    multi::{many0, separated_list1},
    sequence::terminated,
    IResult,
};
fn parse_number_block(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(line_ending, map_res(digit1, u64::from_str))(input.trim_start())
}

pub fn parse_calories(input: &str) -> Vec<Vec<u64>> {
    let (_, calories) = many0(terminated(parse_number_block, opt(newline)))(input).unwrap();
    calories
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_calories(include_str!("test_example")),
            vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]]
        )
    }
}
