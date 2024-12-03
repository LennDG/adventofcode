use regex::{self, Regex};

fn main() {
    let input = include_str!("input");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let matches = re.captures_iter(input);

    let mut sum = 0;
    for m in matches {
        sum += m["first"].parse::<i32>().unwrap() * m["second"].parse::<i32>().unwrap();
    }

    println!("{}", sum);
}

fn part_two(input: &str) {
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)|do\(\)|don't\(\)").unwrap();
    let matches = re.captures_iter(input);

    let mut sum = 0;
    let mut enabled = true;
    for m in matches {
        let matched = m.get(0).unwrap().as_str();
        if matched == "do()" {
            enabled = true;
        } else if matched == "don't()" {
            enabled = false;
        } else if enabled {
            sum += m["first"].parse::<i32>().unwrap() * m["second"].parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}
