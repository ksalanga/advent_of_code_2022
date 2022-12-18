// Advent of Code 2022: Day 2 Rock Paper Scissors
// https://adventofcode.com/2022/day/2

use std::fs;

fn main() {
    // Read a file:
    let tournament = fs::read_to_string("../../inputs/day_2/example1.txt").unwrap();
    // Will panic, file opening error unhandled

    let mut total_score = 0;

    tournament.lines().for_each(|round| {
        let hand_inputs: Vec<&str> = round.split_whitespace().collect();
        let opponent_hand_input = hand_inputs[0];
        let my_hand_input = hand_inputs[1];

        total_score += round_score(opponent_hand_input, my_hand_input);
    });

    println!("Total Score: {}", total_score);
}

fn round_score(opponent_input: &str, my_input: &str) -> i32 {
    let my_hand = Hand::new(my_input);
    let opponent_hand = Hand::new(opponent_input);

    let result = my_hand.compare(opponent_hand);

    my_hand.score() + result.score()
}

#[derive(Debug, PartialEq)]
enum Result {
    Won,
    Draw,
    Lost,
}

impl Result {
    fn score(&self) -> i32 {
        match *self {
            Result::Won => 6,
            Result::Draw => 3,
            Result::Lost => 0,
        }
    }
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn new(input: &str) -> Hand {
        match input {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => Hand::Rock, // new fn assumes clean input... catch all case because Rock is definitely wrong if invalid input.
        }
    }

    fn score(&self) -> i32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn compare(&self, opponent: Hand) -> Result {
        match *self {
            Hand::Rock => match opponent {
                Hand::Scissors => Result::Won,
                Hand::Paper => Result::Lost,
                Hand::Rock => Result::Draw,
            },
            Hand::Paper => match opponent {
                Hand::Rock => Result::Won,
                Hand::Scissors => Result::Lost,
                Hand::Paper => Result::Draw,
            },
            Hand::Scissors => match opponent {
                Hand::Paper => Result::Won,
                Hand::Rock => Result::Lost,
                Hand::Scissors => Result::Draw,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Hand;
    use crate::Result;

    #[test]
    fn rock_beats_scissors() {
        let rock = Hand::Rock;

        assert_eq!(rock.compare(Hand::Scissors), Result::Won);
    }

    #[test]
    fn rock_loses_paper() {
        let rock = Hand::Rock;

        assert_eq!(rock.compare(Hand::Paper), Result::Lost);
    }

    #[test]
    fn rock_ties_rock() {
        let rock = Hand::Rock;

        assert_eq!(rock.compare(Hand::Rock), Result::Draw);
    }

    #[test]
    fn paper_beats_rock() {
        let paper = Hand::Paper;

        assert_eq!(paper.compare(Hand::Rock), Result::Won);
    }

    #[test]
    fn paper_loses_scissors() {
        let paper = Hand::Paper;

        assert_eq!(paper.compare(Hand::Scissors), Result::Lost);
    }

    #[test]
    fn paper_ties_paper() {
        let paper = Hand::Paper;

        assert_eq!(paper.compare(Hand::Paper), Result::Draw);
    }

    #[test]
    fn scissors_beats_paper() {
        let scissors = Hand::Scissors;

        assert_eq!(scissors.compare(Hand::Paper), Result::Won);
    }

    #[test]
    fn scissors_loses_rock() {
        let scissors = Hand::Scissors;

        assert_eq!(scissors.compare(Hand::Rock), Result::Lost);
    }

    #[test]
    fn scissors_ties_scissors() {
        let scissors = Hand::Scissors;

        assert_eq!(scissors.compare(Hand::Scissors), Result::Draw);
    }
}
