use std::{collections::HashMap, iter::zip};

fn main() {
    let input = include_str!("input");

    let (first_list, second_list) = parse_input(input);

    //let result = part_one(first_list, second_list);
    //println!("{}", result);

    use std::time::Instant;
    let start = Instant::now();
    let result = part_two(first_list.clone(), second_list.clone());
    let duration = start.elapsed();
    println!("Part 2: {} (took {:?})", result, duration);
}

fn part_one(first_list: Vec<u32>, second_list: Vec<u32>) -> u32 {
    zip(first_list, second_list)
        .map(|(a, b)| (a as i32 - b as i32).abs() as u32)
        .sum()
}

fn part_two(first_list: Vec<u32>, second_list: Vec<u32>) -> u32 {
    let mut number_count = HashMap::new();
    for i in second_list {
        *number_count.entry(i).or_insert(0) += 1;
    }

    first_list
        .iter()
        .map(|x| x * number_count.get(x).unwrap_or(&0))
        .sum()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first_list = vec![];
    let mut second_list = vec![];

    input.lines().for_each(|line| {
        let (first, second) = line.split_once("   ").unwrap();
        first_list.push(first.parse::<u32>().unwrap());
        second_list.push(second.parse::<u32>().unwrap());
    });

    first_list.sort();
    second_list.sort();

    (first_list, second_list)
}
