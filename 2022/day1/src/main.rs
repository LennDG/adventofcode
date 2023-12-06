use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace1, newline, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};
mod parser;
fn main() {
    println!("Hello, world!");
}

// fn parse_input(input: &str) -> Vec<Vec<u32>> {
//     let (_, calories) = separated_list1(newline, terminated(digit1, newline));
// }
