use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple,
    IResult,
};

use super::{CubeSet, Game};

pub fn parse_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();
    let sets: Vec<&str> = parts[1].split(';').collect();

    let (_, id) = parse_game_id(parts[0]).unwrap();
    let sets = sets.into_iter().map(parse_set).collect();

    Game { id, sets }
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (remaining, (_, id)) = tuple((tag("Game "), map_res(digit1, str::parse)))(input)?;
    Ok((remaining, id))
}

// 1 green, 2 blue
fn parse_set(line: &str) -> CubeSet {
    let cubes: Vec<&str> = line.split(',').collect();
    let mut set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for cube in cubes {
        match parse_cube(cube) {
            Ok((color, amount)) => match color.trim() {
                "red" => set.red = amount,
                "green" => set.green = amount,
                "blue" => set.blue = amount,
                _ => (),
            },
            Err(e) => println!("Error {}", e),
        }
    }
    set
}

fn parse_cube(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set("1 green, 205 red, 3 blue"),
            CubeSet {
                red: 205,
                green: 1,
                blue: 3
            }
        );

        assert_eq!(
            parse_set("1 green"),
            CubeSet {
                red: 0,
                green: 1,
                blue: 0
            }
        )
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                sets: vec![
                    CubeSet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }
        )
    }
}
