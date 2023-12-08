use crate::*;
use nom::{
    self,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, char, line_ending, multispace1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

pub fn parse_map(input: &str) -> Map {
    let (_, (directions, nodes)) =
        separated_pair(parse_directions, multispace1, parse_nodes)(input).unwrap();

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    nodes.iter().for_each(|(first, second, third)| {
        node_map.insert(first.to_string(), (second.to_string(), third.to_string()));
    });

    Map {
        directions,
        nodes: node_map,
    }
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map_res(anychar, Direction::try_from))(input)
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    many1(terminated(parse_line, opt(line_ending)))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (first, (second, _, third, _))) = separated_pair(
        alphanumeric1,
        tag(" = ("),
        tuple((alphanumeric1, tag(", "), alphanumeric1, char(')'))),
    )(input.trim())?;

    Ok((input, (first, second, third)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("AAA = (BBB, CCC)\n"),
            Ok(("", ("AAA", "BBB", "CCC")))
        )
    }

    #[test]
    fn test_parse_nodes() {
        assert_eq!(
            parse_nodes("11A = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)"),
            Ok((
                "",
                vec![
                    ("11A", "BBB", "BBB"),
                    ("BBB", "AAA", "ZZZ"),
                    ("ZZZ", "ZZZ", "ZZZ")
                ]
            ))
        )
    }
}
