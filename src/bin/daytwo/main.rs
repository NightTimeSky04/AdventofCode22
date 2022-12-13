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
        let opponent_hand_shape_score = self.opponent_handshape.score_hand_shape();
        let response_hand_shape_score = self.suggested_response.score_hand_shape();

        let win_draw_loss_score = if opponent_hand_shape_score == response_hand_shape_score {
            3
        } else if opponent_hand_shape_score % 3 == response_hand_shape_score - 1 {
            6
        } else {
            0
        };

        let round_score = response_hand_shape_score + win_draw_loss_score;

        round_score
    }
}

fn main() {
    let strategy_guide_input =
        fs::read_to_string("./src/bin/daytwo/strategy_guide.txt").expect("Problem reading file.");

    let strategy_guide = strategy_guide_input
        .lines()
        .map(|round| round.parse::<Round>().unwrap());

    let total_score: u32 = strategy_guide.map(|round| round.score_round()).sum();

    println!("Total score achieved when following the strategy guide: {total_score}");
}
