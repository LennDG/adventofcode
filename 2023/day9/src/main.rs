fn main() {
    let input = include_str!("input");

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|sequence| {
            sequence
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    part_one(sequences.clone());
    part_two(sequences)
}

fn part_one(sequences: Vec<Vec<i64>>) {
    let total = sequences.iter().fold(0, |total, s| {
        let diffs = calc_diffs(s);
        total + calc_next(diffs)
    });

    println!("Total of all predictions: {total}")
}

fn part_two(sequences: Vec<Vec<i64>>) {
    let total = sequences.iter().fold(0, |total, s| {
        let diffs = calc_diffs(s);
        total + calc_prev(diffs)
    });

    println!("Total of all predictions: {total}")
}

fn calc_diffs(s: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs: Vec<Vec<i64>> = vec![s.to_owned()];
    let mut diff = s.to_owned();

    while diff.iter().any(|d| *d != 0) {
        diff = calc_diff(&diff);
        diffs.push(diff.clone())
    }

    diffs
}

fn calc_diff(sequence: &Vec<i64>) -> Vec<i64> {
    sequence[..sequence.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, s)| sequence[i + 1] - s)
        .collect()
}

fn calc_next(mut diffs: Vec<Vec<i64>>) -> i64 {
    diffs.reverse();
    diffs
        .iter()
        .fold(0, |next, diff| next + diff.last().unwrap())
}

fn calc_prev(mut diffs: Vec<Vec<i64>>) -> i64 {
    diffs.reverse();
    diffs.iter().fold(0, |prev, diff| diff[0] - prev)
}
