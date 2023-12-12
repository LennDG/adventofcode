use std::cmp;

fn main() {
    let input = include_str!("input");
    let universe = parse(input);
    part_one(universe.clone());
    part_two(universe)
}

fn part_one(mut universe: Universe) {
    universe = expand_universe_one(universe);

    let galaxies = universe.get_galaxy_coordinates();

    let distance_sum: usize = galaxies[..galaxies.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, (x, y))| {
            galaxies[i + 1..galaxies.len()]
                .iter()
                .map(|(x2, y2)| x.abs_diff(*x2) + y.abs_diff(*y2))
        })
        .sum();

    println!("{distance_sum}");
}

fn part_two(universe: Universe) {
    let galaxies = universe.get_galaxy_coordinates();

    let distance_sum: usize = galaxies[..galaxies.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            galaxies[i + 1..galaxies.len()]
                .iter()
                .map(|(x2, y2)| calc_expanded_distance((*x1, *y1), (*x2, *y2), 1000000, &universe))
        })
        .sum();

    println!("{distance_sum}");
}

fn calc_expanded_distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    expansion_factor: usize,
    universe: &Universe,
) -> usize {
    let crossed_expansion_rows = (cmp::min(y1, y2)..cmp::max(y1, y2))
        .filter(|y| universe.expansion_rows.contains(y))
        .count();

    let crossed_expansion_columns = (cmp::min(x1, x2)..cmp::max(x1, x2))
        .filter(|x| universe.expansion_columns.contains(x))
        .count();

    (x1.abs_diff(x2) - crossed_expansion_columns)
        + (expansion_factor * crossed_expansion_columns)
        + (y1.abs_diff(y2) - crossed_expansion_rows)
        + (expansion_factor * crossed_expansion_rows)
}

fn parse(input: &str) -> Universe {
    let universe: Vec<Vec<Option<Galaxy>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { Some(Galaxy) } else { None })
                .collect()
        })
        .collect();

    Universe::new(universe)
}

fn expand_universe_one(universe: Universe) -> Universe {
    // insert horizontal
    let mut expanded_universe: Vec<Vec<Option<Galaxy>>> = vec![];
    for line in universe.universe {
        expanded_universe.push(line.clone());
        if line.iter().all(|point| point.is_none()) {
            expanded_universe.push(line.clone())
        }
    }

    // insert vertical
    let empty_columns: Vec<usize> = (0..expanded_universe[0].len())
        .filter(|i| expanded_universe.iter().all(|line| line[*i].is_none()))
        .collect();

    for line in expanded_universe.iter_mut() {
        for i in empty_columns.iter().rev() {
            line.insert(*i, None)
        }
    }

    Universe::new(expanded_universe)
}

#[derive(Debug, Clone)]
struct Universe {
    universe: Vec<Vec<Option<Galaxy>>>,
    expansion_columns: Vec<usize>,
    expansion_rows: Vec<usize>,
}

impl Universe {
    fn new(galaxies: Vec<Vec<Option<Galaxy>>>) -> Self {
        Universe {
            universe: galaxies,
            expansion_columns: vec![],
            expansion_rows: vec![],
        }
        .set_expansion_columns()
        .set_expansion_rows()
    }

    fn set_expansion_rows(mut self) -> Self {
        self.expansion_rows = self
            .universe
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|point| point.is_none()))
            .map(|(i, _)| i)
            .collect();

        self
    }

    fn set_expansion_columns(mut self) -> Self {
        self.expansion_columns = (0..self.universe[0].len())
            .filter(|i| self.universe.iter().all(|line| line[*i].is_none()))
            .collect();

        self
    }

    fn get_galaxy_coordinates(&self) -> Vec<(usize, usize)> {
        self.universe
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, tile)| tile.map(|_| (x, y)))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Galaxy;
