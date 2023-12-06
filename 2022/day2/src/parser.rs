use nom::{
    self,
    character::complete::{alpha1, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::{Move, Outcome, Round};

pub fn parse_rounds_outcomes(input: &str) -> Vec<(Move, Outcome)> {
    let (_, rounds) = separated_list1(newline, parse_round_outcome)(input).unwrap();
    rounds
}

fn parse_round_outcome(input: &str) -> IResult<&str, (Move, Outcome)> {
    let (remaining, (theirs, _, outcome)) = tuple((
        map_res(alpha1, Move::try_from),
        multispace1,
        map_res(alpha1, Outcome::try_from),
    ))(input.trim())?;

    Ok((remaining, (theirs, outcome)))
}

pub fn parse_rounds(input: &str) -> Vec<Round> {
    let (_, rounds) = separated_list1(newline, parse_round)(input).unwrap();
    rounds
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (remaining, (theirs, _, ours)) = tuple((
        map_res(alpha1, Move::try_from),
        multispace1,
        map_res(alpha1, Move::try_from),
    ))(input.trim())?;

    Ok((remaining, Round { ours, theirs }))
}
