fn main() {
    let input = include_str!("input");
    let moves = input
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_once(" ").unwrap();
            let amount = amount.parse().unwrap();
            match direction {
                "forward" => Move::Forward(amount),
                "down" => Move::Down(amount),
                "up" => Move::Up(amount),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    part_one(&moves);
    part_two(&moves);
}

fn part_one(moves: &[Move]) {
    let mut position = (0, 0);
    for command in moves {
        match command {
            Move::Forward(amount) => position.0 += amount,
            Move::Down(amount) => position.1 += amount,
            Move::Up(amount) => position.1 -= amount,
        }
    }
    println!("{}", position.0 * position.1);
}

fn part_two(moves: &[Move]) {
    let mut position = (0, 0);
    let mut aim = 0;

    for command in moves {
        match command {
            Move::Forward(amount) => {
                position.0 += amount;
                position.1 += amount * aim;
            }
            Move::Down(amount) => aim += amount,
            Move::Up(amount) => aim -= amount,
        }
    }

    println!("{}", position.0 * position.1);
}

enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}
