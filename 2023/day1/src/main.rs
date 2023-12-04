#![allow(dead_code)]
use nom::Slice;
fn main() {
    let input = include_str!("input");
    //part_one(input);
    part_two(input);
}

// zero one two three four five six seven eight nine
// Could write a function that moves throught the line one character at a time
// and tries to parse the words
// but just find-replace is way easier
// replacing is actually not 100% correct when digits share a letter (like eightwothree is 823 and not eigh23)
fn part_two(input: &str) {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        println!("{line}");
        let digits = find_digits(line);
        println!("{digits}");
        sum += digits;
    }

    println!("The total sum for part 2 is {sum}")
}

fn find_digits(line: &str) -> u32 {
    let mut first_digit: u8 = 0;
    let mut last_digit: u8 = 0;

    // Get the actual words
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    'outer: for n in 1..line.len() + 1 {
        let first_n = line.slice(0..n);
        if first_n.chars().last().unwrap().is_ascii_digit() {
            first_digit =
                u8::try_from(first_n.chars().last().unwrap().to_digit(10).unwrap()).unwrap();
            break;
        }
        for (i, word) in words.into_iter().enumerate() {
            if first_n.contains(word) {
                first_digit = u8::try_from(i).unwrap();
                break 'outer;
            }
        }
    }

    'outer: for n in 0..line.len() {
        let last_n = line.slice((line.len() - n - 1)..line.len());
        //println!("{last_n}");
        if last_n.chars().next().unwrap().is_ascii_digit() {
            last_digit =
                u8::try_from(last_n.chars().next().unwrap().to_digit(10).unwrap()).unwrap();
            break;
        }
        for (i, word) in words.into_iter().enumerate() {
            if last_n.contains(word) {
                last_digit = u8::try_from(i).unwrap();
                break 'outer;
            }
        }
    }

    println!("First digit {first_digit}, Last digit {last_digit}");

    format!("{first_digit}{last_digit}").parse::<u32>().unwrap()
}

fn part_one(input: &str) {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let line_number = get_line_number(line);
        sum += line_number;
    }

    println!("The sum for part one is: {sum}")
}

fn get_line_number(line: &str) -> u32 {
    let first_digit = line.as_bytes()[line.find(|c: char| c.is_ascii_digit()).unwrap()] as char;
    let last_digit = line.as_bytes()[line.rfind(|c: char| c.is_ascii_digit()).unwrap()] as char;
    format!("{first_digit}{last_digit}").parse::<u32>().unwrap()
}

fn read_input_fs_err() -> Result<String, std::io::Error> {
    // now from the `fs-err` crate, rather than `std::fs`
    fs_err::read_to_string("input")
}
