use crate::*;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace1, newline, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

pub fn parse_almanac(input: &str) -> Almanac {
    let (_, (seeds, _, maps)) =
        tuple((parse_seed_list, multispace1, many1(parse_map)))(input).unwrap();

    let mut almanac_maps: Vec<Map> = vec![];
    for map in maps {
        let mut map_lines: Vec<MapLine> = vec![];
        for line in map {
            map_lines.push(MapLine {
                destination_range_start: line[0],
                source_range_start: line[1],
                range_length: line[2],
            });
        }
        almanac_maps.push(Map { lines: map_lines })
    }

    Almanac {
        seeds,
        maps: almanac_maps,
    }
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, map_res(digit1, u64::from_str))(input.trim())
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (remaining, (_, seeds)) = tuple((tag("seeds: "), parse_number_list))(input)?;
    Ok((remaining, seeds))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (remaining, (_, _, _, maplines)) = tuple((
        take_until("map:"),
        tag("map:"),
        newline,
        many1(parse_number_list),
    ))(input)?;

    Ok((remaining, maplines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_map(include_str!("test_parse_map")),
            Ok(("", vec![vec![50, 98, 2], vec![52, 50, 48]]))
        )
    }
}
