mod parser;

fn main() {
    let input = include_str!("example");
    let rounds = parser::parse_rounds(input);
    part_one(rounds);

    let round_outcomes = parser::parse_rounds_outcomes(input);
    part_two(round_outcomes)
}

fn part_one(rounds: Vec<Round>) {
    let total_points: u32 = rounds.iter().map(|round| round.score()).sum();
    println!("Total round points is {total_points}")
}

fn part_two(round_outcomes: Vec<(Move, Outcome)>) {
    let total_points: u32 = round_outcomes
        .iter()
        .map(|(theirs, outcome)| Round {
            theirs: *theirs,
            ours: theirs.required_move(*outcome),
        })
        .map(|round| round.score())
        .sum();

    println!("Total round points is {total_points}")
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn inherent_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(&self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn winning_move(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn losing_move(self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn required_move(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.winning_move(),
            Outcome::Draw => self,
            Outcome::Loss => self.losing_move(),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<&str> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl Outcome {
    fn points(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Round {
    ours: Move,
    theirs: Move,
}

impl Round {
    fn score(self) -> u32 {
        self.ours.inherent_points() + self.outcome().points()
    }

    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    // Original code
    fn round_points(&self) -> u32 {
        let mut points = self.ours.inherent_points();
        // draw
        if self.ours == self.theirs {
            points += 3;
        }

        match self.ours {
            Move::Rock => {
                if self.theirs == Move::Scissors {
                    points += 6
                }
            }
            Move::Paper => {
                if self.theirs == Move::Rock {
                    points += 6
                }
            }
            Move::Scissors => {
                if self.theirs == Move::Paper {
                    points += 6
                }
            }
        }

        points
    }
}
