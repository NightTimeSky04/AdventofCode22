// Work In Progress!

use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct InputError;

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input contains a character that does not correspond to a hand shape."
        )
    }
}
impl std::error::Error for InputError {}

#[derive(Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for HandShape {
    type Err = InputError;
    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "A" | "X" => Ok(HandShape::Rock),
            "B" | "Y" => Ok(HandShape::Paper),
            "C" | "Z" => Ok(HandShape::Scissors),
            _ => Err(InputError),
        }
    }
}

impl HandShape {
    fn score_hand_shape(self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

struct Round {
    opponent_handshape: HandShape,
    suggested_response: HandShape,
}

impl FromStr for Round {
    type Err = InputError;

    fn from_str(letter_pair: &str) -> Result<Self, Self::Err> {
        let round: Vec<_> = letter_pair.split_whitespace().collect();

        let opponent_handshape = round[0].parse()?;
        let suggested_response = round[1].parse()?;

        Ok(Round {
            opponent_handshape,
            suggested_response,
        })
    }
}

impl Round {
    fn score_round(self) -> u32 {
        let hand_shape_score = self.suggested_response.score_hand_shape();

        let total_score = hand_shape_score;

        total_score
    }
}

fn main() {
    let strategy_guide_input =
        fs::read_to_string("./src/bin/daytwo/strategy_guide.txt").expect("Problem reading file.");

    let strategy_guide = strategy_guide_input
        .lines()
        .map(|round| round.parse::<Round>().unwrap());

    for round in strategy_guide {
        println!(
            "{:?} vs {:?}",
            round.opponent_handshape, round.suggested_response
        )
    }
}
