// Work In Progress! (early stages!)

use std::fs;

enum HandShape {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let strategy_guide = fs::read_to_string("./src/bin/daytwo/strategy_guide.txt");
}

fn score_hand_shapes(hand_shape: HandShape) -> u32 {
    let score = match hand_shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };

    score
}
