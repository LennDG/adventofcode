mod parser;

fn main() {
    let input = include_str!("example");
    let games: Vec<Game> = input.lines().map(parser::parse_line).collect();
    part_one(games.clone());
    part_two(games)
}

// maximums: 12 red cubes, 13 green cubes, and 14 blue cubes
fn part_one(games: Vec<Game>) {
    let mut sum = 0;
    'game: for game in games {
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                continue 'game;
            }
        }
        sum += game.id;
    }

    println!("The sum of IDs for possible games is: {sum}")
}

fn part_two(games: Vec<Game>) {
    let mut sum = 0;
    for game in games {
        let max_red = game.sets.iter().map(|s| s.red).max().unwrap_or(0);
        let max_green = game.sets.iter().map(|s| s.green).max().unwrap_or(0);
        let max_blue = game.sets.iter().map(|s| s.blue).max().unwrap_or(0);

        sum += max_red * max_green * max_blue;
    }

    println!("The sum of powers of minimal sets is: {sum}")
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}
