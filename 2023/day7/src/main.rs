use crate::parser::parse_hands;
use std::collections::HashMap;

mod parser;
fn main() {
    let input = include_str!("input");
    let hands = parse_hands(input);

    part_two(hands)
}

// Part one does not give the correct answer after refactoring for part two
fn part_two(mut hands: Vec<Hand>) {
    hands.sort();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1));

    println!("Total winnings: {total_winnings}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(color_eyre::eyre::eyre!("not a valid card: {c:?}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            core::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match &self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return *ordering,
                    }
                }
                panic!("The same!")
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(core::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    match &self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return Some(*ordering),
                    }
                }
                panic!("The same!")
            }
            ord => ord,
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, u32> = HashMap::new();

        for card in &self.cards {
            match counts.get(card) {
                None => counts.insert(*card, 1),
                Some(count) => counts.insert(*card, *count + 1),
            };
        }

        let j_counts = counts.remove(&Card::J);
        let mut totals: Vec<u32> = counts.values().cloned().collect();
        totals.sort();
        totals.reverse();

        if let Some(c) = j_counts {
            if c < 5 {
                totals[0] += c;
            } else {
                return HandType::FiveOfAKind;
            }
        }

        let totals_length = totals.len();
        match totals[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match totals_length {
                2 => HandType::FullHouse,
                3 => HandType::ThreeOfAKind,
                _ => panic!("There should not be more than 5 cards!"),
            },
            2 => match totals_length {
                3 => HandType::TwoPair,
                4 => HandType::OnePair,
                _ => panic!("There should not be more than 5 cards!"),
            },
            1 => HandType::HighCard,
            v => panic!(
                "There should not be more than 5 cards! {v} cards found for {:?}",
                &self
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
