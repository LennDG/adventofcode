use std::str::FromStr;

fn parse_calories(input: &str) -> Vec<Vec<u64>> {
    let (_, calories) = many0(parse_number_block)(input).unwrap();
    //let (_, calories) = separated_list1(line_ending, parse_number_block)(input).unwrap();
    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_calories(include_str!("test_example")),
            vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]]
        )
    }
}
