use itertools::Itertools;

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

pub(crate) fn exec() -> u32 {
    let contents = include_str!("../inputs/2.txt");
    contents.split('\n').into_iter().fold(0, |acc, strategy| {
        let v = strategy.split(' ').collect_vec();
        let opponent_move: RPSChoice = v.first().expect("").to_owned().into();
        let my_move: RPSChoice = v.get(1).expect("").to_owned().into();

        let outcome = perform_round(&my_move, &opponent_move);

        acc + my_move.score() + outcome.score()
    })
}
