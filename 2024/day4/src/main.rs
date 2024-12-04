fn main() {
    let input = include_str!("input");
    // Pad to avoid having to think for edge cases
    let padded = pad_input(input);

    part_one(padded.clone());
    part_two(padded);
}

fn pad_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // 4 empty at top, bottom, left and right
    let mut padded = vec![vec![' '; lines[0].len() + 8]; lines.len() + 8];

    for (y, line) in lines.into_iter().enumerate() {
        padded[y + 4][4..line.len() + 4].copy_from_slice(&line);
    }

    padded
}

fn part_one(input: Vec<Vec<char>>) {
    // Approach:
    // Look for Xs
    // When finding an X, go in all 8 cardinal directions to find M, A, S in order.
    // If you find them in order, add 1 to your score.
    // Return the score.
    let mut score = 0;

    let directions = [
        (0, 1),   // L -> R
        (1, 1),   // R -> D
        (1, 0),   // T -> D
        (1, -1),  // L -> D
        (0, -1),  // R -> L
        (-1, -1), // L -> T
        (-1, 0),  // D -> T
        (-1, 1),  // R -> T
    ];

    for (y, line) in input.iter().skip(4).take(input.len() - 4).enumerate() {
        let y = y + 4;
        for (x, c) in line.iter().skip(4).take(line.len() - 4).enumerate() {
            let x = x + 4;
            if *c == 'X' {
                score += directions
                    .iter()
                    .filter(|&&(dy, dx)| check_xmas(&input, y, x, dy, dx))
                    .count();
            }
        }
    }

    println!("Part one: {}", score);
}

fn check_xmas(input: &[Vec<char>], y: usize, x: usize, dy: i32, dx: i32) -> bool {
    let positions = [(1, 'M'), (2, 'A'), (3, 'S')];

    positions.iter().all(|&(step, expected)| {
        let ny = (y as i32 + dy * step) as usize;
        let nx = (x as i32 + dx * step) as usize;
        input[ny][nx] == expected
    })
}

fn part_two(input: Vec<Vec<char>>) {
    let mut score = 0;

    for (y, line) in input.iter().skip(4).take(input.len() - 4).enumerate() {
        let y = y + 4;
        for (x, c) in line.iter().skip(4).take(line.len() - 4).enumerate() {
            let x = x + 4;
            if *c == 'A' {
                if check_x_mas(&input, y, x) {
                    score += 1;
                }
            }
        }
    }

    println!("Part two: {}", score);
}

fn check_x_mas(input: &[Vec<char>], y: usize, x: usize) -> bool {
    let positions_1 = [(-1, 'M'), (1, 'S')];
    let positions_2 = [(-1, 'S'), (1, 'M')];

    let check_diagonal = |y_modifier: fn(i32) -> i32| {
        positions_1.iter().all(|&(d, expected)| {
            input[(y as i32 + y_modifier(d)) as usize][(x as i32 + d) as usize] == expected
        }) || positions_2.iter().all(|&(d, expected)| {
            input[(y as i32 + y_modifier(d)) as usize][(x as i32 + d) as usize] == expected
        })
    };

    // TL -> BR uses identity function (y + d)
    let first = check_diagonal(|d| d);
    // BL -> TR uses negation function (y - d)
    let second = check_diagonal(|d| -d);

    first && second
}
