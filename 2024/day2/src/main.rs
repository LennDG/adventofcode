fn main() {
    let input = include_str!("input");
    let lines = parse_input(input);
    println!("{}", part_one(&lines));

    use std::time::Instant;
    let start = Instant::now();
    let result = part_two(&lines);
    let duration = start.elapsed();

    println!("{}", result);
    println!("Time taken: {:?}", duration);
}

fn part_one(lines: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for line in lines {
        // Check if the line is safe.
        if check_safe(line) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn part_two(lines: &Vec<Vec<i32>>) -> i32 {
    // Going to keep this one pretty simple.
    let mut safe_reports = 0;

    for line in lines {
        // Check if the line is safe.
        if check_safe(line) {
            safe_reports += 1;
        } else {
            // let mut temp = Vec::with_capacity(line.len() - 1);
            // for skip_idx in 0..line.len() {
            //     temp.clear();
            //     temp.extend_from_slice(&line[..skip_idx]);
            //     temp.extend_from_slice(&line[skip_idx + 1..]);
            //     if check_safe(&temp) {
            //         safe_reports += 1;
            //         break;
            //     }
            // }

            // This one is faster!
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);
                if check_safe(&new_line) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

fn check_safe(line: &Vec<i32>) -> bool {
    // Set the direction of the report.
    let mut increasing = true;
    if line[0] - line[1] > 0 {
        increasing = false;
    } else if line[0] - line[1] == 0 {
        return false;
    }

    for i in 1..line.len() {
        let num = line[i];
        let previous = line[i - 1];

        // Direction check.
        if increasing && num - previous < 0 {
            return false;
        } else if !increasing && num - previous > 0 {
            return false;
        } else if num - previous == 0 {
            return false;
        }

        // Difference check.
        if (num - previous).abs() > 3 {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
