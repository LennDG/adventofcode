use crate::*;
use nom::{
    self,
    character::complete::{anychar, char, digit1, multispace1, newline},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub fn parse_hands(input: &str) -> Vec<Hand> {
    let (_, hands) = separated_list1(
        newline,
        separated_pair(parse_cards, multispace1, map_res(digit1, u32::from_str)),
    )(input)
    .unwrap();

    hands
        .iter()
        .map(|(cards, bid)| Hand {
            cards: cards.clone(),
            bid: *bid,
        })
        .collect()
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    many1(map_res(anychar, Card::try_from))(input)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_hands() {

//     }
// }
