use nom::{
    character::{
        complete::space0,
        complete::{char, digit1, multispace1, newline, space1},
    },
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};
use std::str::FromStr;

use crate::Board;

pub fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (_, (numbers, boards)) =
        separated_pair(parse_numbers, multispace1, parse_boards)(input).unwrap();

    (numbers, boards)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(char(','), map_res(digit1, u32::from_str))(input)
}

fn parse_boards(input: &str) -> IResult<&str, Vec<Board>> {
    let (res, boards) = separated_list1(
        pair(newline, newline),
        separated_list1(newline, parse_board_line),
    )(input)
    .unwrap();

    let boards = boards
        .into_iter()
        .map(|board| Board::from(board))
        .collect::<Vec<_>>();

    Ok((res, boards))
}

fn parse_board_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?; // Add this line to consume optional leading spaces
    separated_list1(space1, map_res(digit1, u32::from_str))(input)
}
