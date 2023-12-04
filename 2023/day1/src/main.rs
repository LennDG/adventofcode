#![allow(dead_code)]

use std::u32::MAX;
fn main() {
    let input = include_str!("example1");
    part_one(input);
    // part_two(input);
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
        let replaced = replace_words(line);
        println!("{replaced}");
        let line_number = get_line_number(&replaced);
        println!("{line_number}");
        sum += line_number;
    }

    println!("The total sum for part 2 is {sum}")
}

fn find_digits(line: &str) -> String {
    let first_digit_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let last_digit_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();


    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first_word_idx = MAX;
    let mut last_word_idx = MAX;
    for word in words {
        match line.find(word) {
            Some(idx) => {
                if idx < first_word_idx
            }
        }
    }

    let first_word_idx = line.find()
    todo!()
}

fn replace_words(line: &str) -> String {
    let mut line = line.replace("zero", "0");
    line = line.replace("one", "1");
    line = line.replace("two", "2");
    line = line.replace("three", "3");
    line = line.replace("four", "4");
    line = line.replace("five", "5");
    line = line.replace("six", "6");
    line = line.replace("seven", "7");
    line = line.replace("eight", "8");
    line = line.replace("nine", "9");
    line
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
