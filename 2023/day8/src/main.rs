use std::collections::HashMap;

mod parser;

fn main() {
    let input = include_str!("input");
    let map = parser::parse_map(input);
    part_two(map)
}

fn part_one(map: Map) {
    let mut count = 0;
    let mut current_node = "AAA".to_string();
    while current_node != "ZZZ" {
        for direction in &map.directions {
            match direction {
                Direction::Left => current_node = map.nodes.get(&current_node).unwrap().0.clone(),
                Direction::Right => current_node = map.nodes.get(&current_node).unwrap().1.clone(),
            }
            count += 1;
            if current_node == "ZZZ" {
                break;
            }
        }
    }

    println!("Total number of steps is: {count}")
}

fn part_two(map: Map) {
    let starts: Vec<String> = map
        .nodes
        .keys()
        .cloned()
        .filter(|node| node.ends_with('A'))
        .collect();

    let ends: Vec<String> = map
        .nodes
        .keys()
        .cloned()
        .filter(|node| node.ends_with('Z'))
        .collect();

    println!("{ends:?}");

    let current_nodes = starts.clone();
    let mut counts: Vec<u32> = vec![];
    for node in current_nodes {
        let mut current_node = node;
        let mut count = 0;
        while !ends.contains(&current_node) {
            for direction in &map.directions {
                match direction {
                    Direction::Left => {
                        current_node = map.nodes.get(&current_node).unwrap().0.clone()
                    }
                    Direction::Right => {
                        current_node = map.nodes.get(&current_node).unwrap().1.clone()
                    }
                }
                count += 1;
                if ends.contains(&current_node) {
                    break;
                }
            }
        }
        counts.push(count);
    }
    println!("{counts:?}");

    println!("LCM of all counts is: {}", lcm(counts))
}

fn lcm(ints: Vec<u32>) -> u64 {
    let ints_64: Vec<u64> = ints.iter().map(|i| *i as u64).collect();
    let mut result = ints_64[0];

    for &num in &ints_64[1..] {
        result = num::integer::lcm(result, num);
    }
    result
}

fn part_two_brute_force(map: Map) {
    let mut count = 0;

    let starts: Vec<String> = map
        .nodes
        .keys()
        .cloned()
        .filter(|node| node.ends_with('A'))
        .collect();

    println!("{starts:?}");

    let ends: Vec<String> = map
        .nodes
        .keys()
        .cloned()
        .filter(|node| node.ends_with('Z'))
        .collect();
    println!("{ends:?}");

    let mut current_nodes = starts.clone();
    while !current_nodes.iter().all(|node| ends.contains(node)) {
        for direction in &map.directions {
            match direction {
                Direction::Left => {
                    current_nodes = current_nodes
                        .iter()
                        .map(|node| map.nodes.get(node).unwrap().0.clone())
                        .collect()
                }
                Direction::Right => {
                    current_nodes = current_nodes
                        .iter()
                        .map(|node| map.nodes.get(node).unwrap().1.clone())
                        .collect()
                }
            }

            count += 1;
            if current_nodes.iter().all(|node| ends.contains(node)) {
                break;
            }
        }
    }

    println!("Total number of steps is: {count}")
}

#[derive(Debug)]
pub struct Map {
    nodes: HashMap<String, (String, String)>,
    directions: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(color_eyre::eyre::eyre!("not a valid direction: {c:?}")),
        }
    }
}

// #[derive(Debug)]
// pub struct Node {
//     name: String,
//     left: Box<Node>,
//     right: Box<Node>,
//     start: bool,
//     end: bool,
// }
