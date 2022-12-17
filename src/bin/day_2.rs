// Advent of Code 2022: Day 2 Rock Paper Scissors
// https://adventofcode.com/2022/day/2

use std::fs;

fn main() {
    // Read a file:
    let tournament = fs::read_to_string("../../inputs/day_1/example2.txt").unwrap();
    // Will panic, file opening error unhandled

    let total_score = 0;

    tournament.lines().for_each(|round| {
        let hands: Vec<&str> = round.split_whitespace().collect();
        let opponent_hand = hands[0];
        let my_hand = hands[1];
    })
}

fn calculate_score(opponent_hand: &str, my_hand: &str) -> i32 {
    let round_score = 0;
    69
}

enum Hand {
    Rock,
    Paper = 2,
    Scissors = 3,
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
}
