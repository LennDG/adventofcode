mod parser;

fn main() {
    let input = include_str!("example");
    let cards: Vec<Card> = input.lines().map(parser::parse_card).collect();
    println!("{:?}", cards)
}

#[derive(Debug, PartialEq)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>,
}
