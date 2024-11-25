fn main() {
    let input = include_str!("input");
    let measurements = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", part_one(&measurements));
    println!("{}", part_two(&measurements));
}

// Count the number of times a measurement is larger than the last
fn part_one(measurements: &[i32]) -> i32 {
    let mut last = 99999999;
    let mut count = 0;
    for &measurement in measurements {
        if measurement > last {
            count += 1;
        }
        last = measurement;
    }
    count
}

// Sliding window of 3 measurements
fn part_two(measurements: &[i32]) -> i32 {
    let mut last = 99999999;
    let mut count = 0;
    for i in 0..measurements.len() - 2 {
        let sum = measurements[i] + measurements[i + 1] + measurements[i + 2];
        if sum > last {
            count += 1;
        }
        last = sum;
    }
    count
}
