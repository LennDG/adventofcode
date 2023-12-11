fn main() {
    let input = include_str!("input");

    let border_width = input.lines().next().unwrap().len() + 2;
    let border = vec![Tile::try_from('.').unwrap(); border_width];
    let mut tiles: Vec<Vec<Tile>> = vec![border.clone()];

    for line in input.lines() {
        let mut tiles_line = vec![Tile::try_from('.').unwrap()];
        let tiles_parsed: Vec<Tile> = line.chars().map(|c| Tile::try_from(c).unwrap()).collect();
        tiles_line.extend_from_slice(&tiles_parsed);
        tiles_line.push(Tile::try_from('.').unwrap());

        tiles.push(tiles_line);
    }

    tiles.push(border);

    let pipes = Pipes::new(tiles);
    //part_one(pipes);
    part_two(pipes)
}

fn part_one(pipes: Pipes) {
    let connecting = pipes.get_connecting_tiles(pipes.start);
    println!("{connecting:?}");

    let path: Vec<(usize, usize)> = build_path(&pipes, pipes.get_connecting_tiles(pipes.start)[0]);
    let length = path.len() / 2;

    println!("Length of the longest: {length}");
}

// Does not work correctly yet!
fn part_two(pipes: Pipes) {
    let path = get_loop(&pipes);
    let mut area = 0;
    for (y, line) in pipes.tiles.iter().enumerate() {
        let mut inside = false;
        let mut half_vertical: Option<Direction> = None;
        for (x, tile) in line.iter().enumerate() {
            if path.contains(&(x, y)) {
                if tile.is_vertical() {
                    inside = !inside;
                } else {
                    match half_vertical {
                        Some(direction) => match direction {
                            Direction::North => {
                                if tile.get_vertical_component() == Some(Direction::South) {
                                    inside = !inside;
                                    half_vertical = None
                                } else if tile.get_vertical_component() == Some(Direction::North) {
                                    half_vertical = None
                                }
                            }
                            Direction::South => {
                                if tile.get_vertical_component() == Some(Direction::North) {
                                    inside = !inside;
                                    half_vertical = None
                                } else if tile.get_vertical_component() == Some(Direction::South) {
                                    half_vertical = None
                                }
                            }
                            _ => (),
                        },
                        None => {
                            if let Some(direction) = tile.get_vertical_component() {
                                half_vertical = Some(direction)
                            }
                        }
                    }
                }
            } else if inside {
                area += 1;
            }
        }
    }

    println!("Total area inside loop is {area}");
}

fn get_loop(pipes: &Pipes) -> Vec<(usize, usize)> {
    build_path(pipes, pipes.get_connecting_tiles(pipes.start)[0])
}

fn build_path(pipes: &Pipes, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut prev = pipes.start;
    let mut current = start;
    let mut path = vec![prev];
    while current != pipes.start {
        let connecting = pipes.get_connecting_tiles(current);
        let next = *connecting
            .iter()
            .filter(|pos| **pos != prev)
            .collect::<Vec<_>>()[0];
        path.push(current);
        prev = current;
        current = next;
    }
    path
}

struct PathTile {
    coordinates: (usize, usize),
    direction: Direction,
}

struct Pipes {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Pipes {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let mut start = (0, 0);
        for (y, line) in tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if tile.is_start() {
                    start = (x, y)
                }
            }
        }

        Pipes { tiles, start }
    }

    fn get_tile(&self, (x, y): (usize, usize)) -> &Tile {
        &self.tiles[y][x]
    }

    fn get_connecting_tiles(&self, (x1, y1): (usize, usize)) -> Vec<(usize, usize)> {
        (x1 - 1..=x1 + 1)
            .flat_map(|x2| (y1 - 1..=y1 + 1).map(move |y2| (x2, y2)))
            .filter(|pos| self.connects((x1, y1), *pos))
            .collect()
    }

    fn connects(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        // rule out anything that is outside of manhatten distance 1
        if x1.abs_diff(x2) + y1.abs_diff(y2) != 1 {
            return false;
        }

        // Determine direction where tile 2 lies
        let direction: Direction;
        if x1 > x2 {
            direction = Direction::West
        } else if x2 > x1 {
            direction = Direction::East
        } else if y1 > y2 {
            direction = Direction::North
        } else {
            direction = Direction::South
        }

        let Tile(directions1) = &self.tiles[y1][x1];
        let Tile(directions2) = &self.tiles[y2][x2];

        match direction {
            Direction::North => {
                directions1.contains(&Direction::North) && directions2.contains(&Direction::South)
            }
            Direction::East => {
                directions1.contains(&Direction::East) && directions2.contains(&Direction::West)
            }
            Direction::South => {
                directions1.contains(&Direction::South) && directions2.contains(&Direction::North)
            }
            Direction::West => {
                directions1.contains(&Direction::West) && directions2.contains(&Direction::East)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Tile(Vec<Direction>);

impl Tile {
    fn is_start(&self) -> bool {
        self.0.len() == 4
    }

    fn is_vertical(&self) -> bool {
        self.0.contains(&Direction::North) && self.0.contains(&Direction::South)
    }

    fn get_vertical_component(&self) -> Option<Direction> {
        if self.0.contains(&Direction::North) {
            Some(Direction::North)
        } else if self.0.contains(&Direction::South) {
            Some(Direction::South)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

// impl Tile {
//     fn connects(&self, other: &Tile) -> bool {}
// }

impl TryFrom<char> for Tile {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile(vec![])),
            'S' => Ok(Tile(vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ])),
            '|' => Ok(Tile(vec![Direction::North, Direction::South])),
            '-' => Ok(Tile(vec![Direction::East, Direction::West])),
            'L' => Ok(Tile(vec![Direction::North, Direction::East])),
            'J' => Ok(Tile(vec![Direction::North, Direction::West])),
            '7' => Ok(Tile(vec![Direction::South, Direction::West])),
            'F' => Ok(Tile(vec![Direction::South, Direction::East])),
            _ => Ok(Tile(vec![])),
        }
    }
}
