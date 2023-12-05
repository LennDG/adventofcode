mod parser;

fn main() {
    let input = include_str!("input");
    let cards: Vec<Card> = input.lines().map(parser::parse_card).collect();
    part_one(cards.clone());
    part_two(cards)
}

fn part_one(cards: Vec<Card>) {
    let total: u32 = cards.iter().map(|card| card.score()).sum();
    println!("The total score of all cards is {total}");
}

fn part_two(mut cards: Vec<Card>) {
    for card in &mut cards {
        card.matches = card.matches();
    }
    let mut copies: Vec<u32> = vec![0; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let current_card_copies = copies[i];
        for copy in &mut copies[i + 1..=i + card.matches] {
            *copy += 1 + current_card_copies;
        }
    }

    let total_cards: u32 = copies.iter().map(|v| v + 1).sum();
    println!("Total number of cards: {total_cards}")
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>,
    matches: usize,
    copies: u32,
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches > 0 {
            2_u32.pow(u32::try_from(matches).unwrap() - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        self.scratched
            .iter()
            .filter(|scratch| self.winning.contains(scratch))
            .count()
    }
}
