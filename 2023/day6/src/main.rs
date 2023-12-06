mod parser;

fn main() {
    let input1 = include_str!("input");
    let races = parser::parse_races(input1);
    println!("{:?}", races);
    part_one(races);

    let input2 = include_str!("input2");
    let (time, distance) = parser::parse_race(input2);
    println!("{time}, {distance}");
    part_two(time, distance)
}

fn part_one(races: Vec<Race>) {
    let mut product = 1;

    for race in races {
        let max_time_held = race.time - 1;

        let options = (1..=max_time_held)
            .filter(|time_held| (race.time - time_held) * time_held > race.distance)
            .count();

        println!("Race can be beaten in {options}");

        product *= options;
    }

    println!("Product of all options: {product}")
}

fn part_two(time: u64, distance: u64) {
    // Binary search shenanigans for fun!
    let start_point = start_point(time, distance);
    println!("Start point found: {start_point}");

    let end_point = end_point(time, distance);
    println!("End point found: {end_point}");

    let options = end_point - start_point + 1;
    println!("Total ways to beat race: {options}");

    // Boring actual math
    let ftime: f64 = time as f64;
    let fdistance: f64 = distance as f64;
    let first =
        ((-ftime + (ftime.powi(2) - 4.0f64 * (-1.0f64) * (-fdistance)).sqrt()) / (-2.0f64)).ceil();
    let second =
        ((-ftime - (ftime.powi(2) - 4.0f64 * (-1.0f64) * (-fdistance)).sqrt()) / (-2.0f64)).ceil();

    println!("First found: {first}");
    println!("Second found: {second}");
}

fn start_point(time: u64, distance: u64) -> u64 {
    let mut l: u64 = 0;
    let mut r = time / 2 - 1;

    while l < r {
        let m = (l + r) / 2;
        let distance_m = (time - m) * m;
        let distance_m_less = (time - m - 1) * (m - 1);
        let distance_m_more = (time - m + 1) * (m + 1);

        match distance_m.cmp(&distance) {
            std::cmp::Ordering::Less => {
                // This is when were just below breakpoint
                if distance_m_less < distance && distance < distance_m_more {
                    return m + 1;
                } else {
                    l = m + 1;
                }
            }
            std::cmp::Ordering::Equal => return m,
            std::cmp::Ordering::Greater => {
                // just above breakpoint
                if distance_m_less < distance && distance < distance_m_more {
                    return m;
                } else {
                    r = m - 1;
                }
            }
        }
    }

    println!("ah fuck");
    0
}

fn end_point(time: u64, distance: u64) -> u64 {
    let mut l: u64 = time / 2 + 1;
    let mut r = time;

    while l < r {
        let m = (l + r) / 2;
        let distance_m = (time - m) * m;
        let distance_m_less = (time - m - 1) * (m - 1);
        let distance_m_more = (time - m + 1) * (m + 1);

        match distance_m.cmp(&distance) {
            std::cmp::Ordering::Less => {
                // This is when were just above breakpoint
                if distance_m_less < distance && distance < distance_m_more {
                    return m - 1;
                } else {
                    r = m - 1;
                }
            }
            std::cmp::Ordering::Equal => return m,
            std::cmp::Ordering::Greater => {
                // just below breakpoint
                if distance_m_less < distance && distance < distance_m_more {
                    return m;
                } else {
                    l = m + 1;
                }
            }
        }
    }

    println!("ah fuck");
    0
}

#[derive(Debug)]
pub struct Race {
    time: u32,
    distance: u32,
}
