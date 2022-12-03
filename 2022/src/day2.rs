use itertools::Itertools;

use crate::Part;

enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

enum GameOutcome {
    Win,
    Draw,
    Loss,
}

impl GameOutcome {
    fn score(&self) -> u32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Draw => 3,
            GameOutcome::Loss => 0,
        }
    }
}

impl From<&str> for GameOutcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => GameOutcome::Loss,
            "Y" => GameOutcome::Draw,
            "Z" => GameOutcome::Win,
            _ => panic!(),
        }
    }
}

impl RPSChoice {
    fn score(&self) -> u32 {
        match self {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3,
        }
    }
}

impl From<&str> for RPSChoice {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

fn perform_round(my_move: &RPSChoice, opponent_move: &RPSChoice) -> GameOutcome {
    match (my_move, opponent_move) {
        (RPSChoice::Rock, RPSChoice::Rock) => GameOutcome::Draw,
        (RPSChoice::Rock, RPSChoice::Paper) => GameOutcome::Loss,
        (RPSChoice::Rock, RPSChoice::Scissors) => GameOutcome::Win,
        (RPSChoice::Paper, RPSChoice::Rock) => GameOutcome::Win,
        (RPSChoice::Paper, RPSChoice::Paper) => GameOutcome::Draw,
        (RPSChoice::Paper, RPSChoice::Scissors) => GameOutcome::Loss,
        (RPSChoice::Scissors, RPSChoice::Rock) => GameOutcome::Loss,
        (RPSChoice::Scissors, RPSChoice::Paper) => GameOutcome::Win,
        (RPSChoice::Scissors, RPSChoice::Scissors) => GameOutcome::Draw,
    }
}

fn get_my_move(opponent_move: &RPSChoice, intended_outcome: &GameOutcome) -> RPSChoice {
    match (opponent_move, intended_outcome) {
        (RPSChoice::Rock, GameOutcome::Win) => RPSChoice::Paper,
        (RPSChoice::Rock, GameOutcome::Draw) => RPSChoice::Rock,
        (RPSChoice::Rock, GameOutcome::Loss) => RPSChoice::Scissors,
        (RPSChoice::Paper, GameOutcome::Win) => RPSChoice::Scissors,
        (RPSChoice::Paper, GameOutcome::Draw) => RPSChoice::Paper,
        (RPSChoice::Paper, GameOutcome::Loss) => RPSChoice::Rock,
        (RPSChoice::Scissors, GameOutcome::Win) => RPSChoice::Rock,
        (RPSChoice::Scissors, GameOutcome::Draw) => RPSChoice::Scissors,
        (RPSChoice::Scissors, GameOutcome::Loss) => RPSChoice::Paper,
    }
}

pub(crate) fn exec(part: &Part) -> u32 {
    let contents = include_str!("../inputs/2.txt");
    contents.split('\n').into_iter().fold(0, |acc, strategy| {
        let v = strategy.split(' ').collect_vec();

        match part {
            Part::One => {
                let opponent_move: RPSChoice = v.first().expect("").to_owned().into();
                let my_move: RPSChoice = v.get(1).expect("").to_owned().into();

                let outcome = perform_round(&my_move, &opponent_move);

                acc + my_move.score() + outcome.score()
            }
            Part::Two => {
                let opponent_move: RPSChoice = v.first().expect("").to_owned().into();
                let intended_outcome: GameOutcome = v.get(1).expect("").to_owned().into();

                let my_move = get_my_move(&opponent_move, &intended_outcome);

                acc + my_move.score() + intended_outcome.score()
            }
        }
    })
}
