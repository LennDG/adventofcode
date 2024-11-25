fn main() {
    let input = include_str!("example");
    let binary: Vec<String> = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();

    part_one(&binary);
    part_two(binary);
}

fn part_one(binary: &[String]) {
    let mut one_count: Vec<u32> = vec![0; 12];
    let mut zero_count: Vec<u32> = vec![0; 12];

    for number in binary {
        for (i, c) in number.chars().enumerate() {
            match c {
                '0' => zero_count[i] += 1,
                '1' => one_count[i] += 1,
                _ => panic!("Invalid binary"),
            }
        }
    }

    // Convert to binary string first
    let gamma: String = one_count
        .iter()
        .zip(zero_count.iter())
        .map(|(ones, zeros)| if ones > zeros { '1' } else { '0' })
        .collect();

    // Convert binary string to decimal (u32)
    let gamma_rate = u32::from_str_radix(&gamma, 2).unwrap();

    // Flip bits for epsilon rate
    let epsilon: String = gamma
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => panic!("Invalid binary"),
        })
        .collect();

    let epsilon_rate = u32::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma_rate * epsilon_rate)
}

fn part_two(binary: Vec<String>) {
    let mut oxygen_numbers = binary.clone();
    let mut co2_numbers = binary;
    let mut oxy_i = 0;
    let mut co2_i = 0;

    // Oxygen: most common bit, 1 in case of tie
    while oxygen_numbers.len() > 1 {
        let bit_count = bit_count(oxygen_numbers, oxy_i);
        if bit_count.one_count < bit_count.zero_count {
            oxygen_numbers = bit_count.zeroes
        } else {
            oxygen_numbers = bit_count.ones
        }

        oxy_i += 1;
    }

    // CO2: least common bit, 0 in case of tie
    while co2_numbers.len() > 1 {
        let bit_count = bit_count(co2_numbers, co2_i);
        if bit_count.one_count < bit_count.zero_count {
            co2_numbers = bit_count.ones
        } else {
            co2_numbers = bit_count.zeroes
        }

        co2_i += 1;
    }

    let oxygen_dec = u32::from_str_radix(&oxygen_numbers[0], 2).unwrap();
    let co2_dec = u32::from_str_radix(&co2_numbers[0], 2).unwrap();

    println!("{}", oxygen_dec * co2_dec)
}

fn bit_count(binary: Vec<String>, i: usize) -> BitCount {
    let mut one_count = 0;
    let mut zero_count = 0;

    let mut ones: Vec<String> = vec![];
    let mut zeroes: Vec<String> = vec![];

    for number in binary {
        match number.as_bytes()[i] as char {
            '0' => {
                zero_count += 1;
                zeroes.push(number);
            }
            '1' => {
                one_count += 1;
                ones.push(number);
            }

            _ => panic!("Invalid binary"),
        }
    }

    BitCount {
        zero_count,
        one_count,
        zeroes,
        ones,
    }
}

struct BitCount {
    zero_count: i32,
    one_count: i32,
    zeroes: Vec<String>,
    ones: Vec<String>,
}
