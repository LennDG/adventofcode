use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::Card;
use std::str::FromStr;

pub fn parse_card(input: &str) -> Card {
    let (_, (_, id, _, winning, _, scratched)) = tuple((
        tuple((tag("Card"), multispace0)),
        map_res(digit1, u32::from_str),
        tuple((multispace0, char(':'), multispace0)),
        parse_number_list,
        tuple((multispace0, char('|'), multispace0)),
        parse_number_list,
    ))(input)
    .unwrap();

    Card {
        id,
        winning,
        scratched,
        matches: 0,
        copies: 0,
    }
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, map_res(digit1, u32::from_str))(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_list() {
        assert_eq!(
            parse_number_list("1 55 3   25 6"),
            Ok(("", vec![1, 55, 3, 25, 6]))
        );

        assert_eq!(
            parse_number_list("  1 21 53 59 44 "),
            Ok(("", vec![1, 21, 53, 59, 44]))
        );
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                scratched: vec![83, 86, 6, 31, 17, 9, 48, 53],
                matches: 0,
                copies: 0
            }
        );
    }
}
