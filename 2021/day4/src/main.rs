use std::collections::HashMap;

mod parser;

fn main() {
    let input = include_str!("input");
    let (numbers, boards) = parser::parse(input);

    //part_one(numbers.clone(), boards.clone());
    part_two(numbers, boards);
}

// Approach:
// Find a first winning line and get its highest index
// For each subsequent line, if any number is higher than that index, eliminate it.
// If all numbers are lower then set it as new winning line + update highest valid index

// Once the board with the winning line has been found, iterate through the list normally to find its score.

// Additional optimization:
// If a line is eliminated, also eliminate the orthogonal line on the invalid number.

// Data structure:
// Get the list of numbers into a hashmap with number-index keypair to make finding the number O(1)

fn part_one(numbers: Vec<u32>, boards: Vec<Board>) {
    let number_map: HashMap<u32, usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, val)| (*val, i))
        .collect();

    let mut highest_idx = numbers.len();
    let mut winning_board = 0;

    for (board_number, board) in boards.iter().enumerate() {
        let mut eliminated_cols: Vec<usize> = vec![];

        for row in &board.rows {
            // Iterate through numbers and break when one is larger than current highest idx.
            // Also note the corresponding column then.
            let mut winning_row = false;
            for (i, number) in row.iter().enumerate() {
                if *number_map.get(number).unwrap_or_else(|| &0) > highest_idx {
                    eliminated_cols.push(i);
                    winning_row = false;
                    break;
                }
                winning_row = true;
            }

            // get the highest_idx of the numbers in the row
            if winning_row {
                highest_idx = row
                    .iter()
                    .map(|number| *number_map.get(number).unwrap())
                    .max()
                    .unwrap();

                println!("Current winning row: {:?}", row);
                println!(
                    "Highest idx: {}, corresponding with: {}",
                    highest_idx, numbers[highest_idx]
                );

                winning_board = board_number
            }
        }

        for (i, column) in board.columns.iter().enumerate() {
            // skip if this column was already eliminated
            if eliminated_cols.contains(&i) {
                break;
            }

            // Iterate through numbers and break when one is larger than current highest idx.
            // Also note the corresponding column then.
            let mut winning_col = false;
            for (i, number) in column.iter().enumerate() {
                if *number_map.get(number).unwrap_or_else(|| &0) > highest_idx {
                    winning_col = false;
                    break;
                }
                winning_col = true;
            }

            // get the highest_idx of the numbers in the row
            if winning_col {
                highest_idx = column
                    .iter()
                    .map(|number| *number_map.get(number).unwrap())
                    .max()
                    .unwrap();

                println!("Current winning column: {:?}", column);
                println!(
                    "Highest idx: {}, corresponding with: {}",
                    highest_idx, numbers[highest_idx]
                );

                winning_board = board_number
            }
        }
    }

    println!(
        "\nHighest idx: {}, corresponding with: {}",
        highest_idx, numbers[highest_idx]
    );
    println!("Winning board number: {}", winning_board);
    println!("winning board: {:?}", boards[winning_board]);

    let board_score = calculate_board_score(&numbers, &boards[winning_board], highest_idx);

    println!("Winning board score: {}", board_score);
}

fn calculate_board_score(numbers: &[u32], board: &Board, highest_idx: usize) -> u32 {
    let mut board_numbers: Vec<&u32> = board.rows.iter().flatten().collect();
    let final_number = numbers[highest_idx];
    println!("{final_number}");

    for number in &numbers[..=highest_idx] {
        if let Some(pos) = board_numbers.iter().position(|bn| *bn == number) {
            board_numbers.swap_remove(pos);
        }
    }

    let sum = board_numbers.into_iter().sum::<u32>();

    println!("{sum}");

    sum * final_number
}

// Going to brute force second part
// Run the part one algorithm, remove the winning board, repeat.
fn part_two(numbers: Vec<u32>, mut boards: Vec<Board>) {
    let number_map: HashMap<u32, usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, val)| (*val, i))
        .collect();

    let mut highest_idx = 100;

    while boards.len() > 1 {
        let (winning, highest) = winning_board(number_map.clone(), boards.clone());
        println!("{winning}, {highest}");
        boards.swap_remove(winning);
        highest_idx = highest
    }

    let score = calculate_board_score(&numbers, &boards[0], highest_idx);

    println!("{}", score);
    println!("{:?}", boards[0])
}

fn winning_board(number_map: HashMap<u32, usize>, boards: Vec<Board>) -> (usize, usize) {
    let mut highest_idx = 100;
    let mut winning_board = 0;

    for (board_number, board) in boards.iter().enumerate() {
        let mut eliminated_cols: Vec<usize> = vec![];

        for row in &board.rows {
            // Iterate through numbers and break when one is larger than current highest idx.
            // Also note the corresponding column then.
            let mut winning_row = false;
            for (i, number) in row.iter().enumerate() {
                if *number_map.get(number).unwrap_or_else(|| &0) > highest_idx {
                    eliminated_cols.push(i);
                    winning_row = false;
                    break;
                }
                winning_row = true;
            }

            // get the highest_idx of the numbers in the row
            if winning_row {
                highest_idx = row
                    .iter()
                    .map(|number| *number_map.get(number).unwrap())
                    .max()
                    .unwrap();

                winning_board = board_number
            }
        }

        for (i, column) in board.columns.iter().enumerate() {
            // skip if this column was already eliminated
            if eliminated_cols.contains(&i) {
                break;
            }

            // Iterate through numbers and break when one is larger than current highest idx.
            // Also note the corresponding column then.
            let mut winning_col = false;
            for (i, number) in column.iter().enumerate() {
                if *number_map.get(number).unwrap_or_else(|| &0) > highest_idx {
                    winning_col = false;
                    break;
                }
                winning_col = true;
            }

            // get the highest_idx of the numbers in the row
            if winning_col {
                highest_idx = column
                    .iter()
                    .map(|number| *number_map.get(number).unwrap())
                    .max()
                    .unwrap();

                winning_board = board_number
            }
        }
    }

    (winning_board, highest_idx)
}

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
}

impl From<Vec<Vec<u32>>> for Board {
    fn from(rows: Vec<Vec<u32>>) -> Self {
        let num_cols = rows[0].len();
        let mut columns = vec![Vec::new(); num_cols];

        // Build columns from rows
        for row in rows.iter() {
            for (col_idx, &num) in row.iter().enumerate() {
                columns[col_idx].push(num);
            }
        }

        Board {
            rows: rows,
            columns,
        }
    }
}
