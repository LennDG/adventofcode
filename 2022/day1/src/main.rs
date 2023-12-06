use itertools::Itertools;

mod parser;
fn main() {
    let input = include_str!("example");
    let calories = parser::parse_calories(input);
    part_one(calories.clone());
    part_two(calories);
}

fn part_one(calories: Vec<Vec<u64>>) {
    let burdendest: u64 = calories.iter().map(|elf| elf.iter().sum()).max().unwrap();
    println!("The burdendest elf has {burdendest} calories");
}

fn part_two(calories: Vec<Vec<u64>>) {
    let top3: u64 = calories
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u64>>()
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("The sum of top 3 burdendest elves is {top3} calories");
}
