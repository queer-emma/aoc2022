use thiserror::Error;

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn to_score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn to_score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Column2 {
    X,
    Y,
    Z,
}

impl Column2 {
    pub fn to_move(&self) -> Move {
        match self {
            Column2::X => Move::Rock,
            Column2::Y => Move::Paper,
            Column2::Z => Move::Scissors,
        }
    }

    pub fn to_outcome(&self) -> Outcome {
        match self {
            Column2::X => Outcome::Lose,
            Column2::Y => Outcome::Draw,
            Column2::Z => Outcome::Win,
        }
    }
}

pub struct Round {
    opponent: Move,
    mine: Column2,
}

#[derive(Debug, Error)]
#[error("unable to parse round: {0}")]
pub struct RoundParseError(String);

#[aoc_generator(day2)]
fn day2_input(input: &str) -> Result<Vec<Round>, RoundParseError> {
    let mut rounds = vec![];

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let opponent_str = *parts
            .get(0)
            .ok_or_else(|| RoundParseError(line.to_owned()))?;
        let mine_str = *parts
            .get(1)
            .ok_or_else(|| RoundParseError(line.to_owned()))?;

        let opponent = match opponent_str {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(RoundParseError(line.to_owned())),
        };
        let mine = match mine_str {
            "X" => Column2::X,
            "Y" => Column2::Y,
            "Z" => Column2::Z,
            _ => return Err(RoundParseError(line.to_owned())),
        };

        rounds.push(Round { opponent, mine })
    }

    Ok(rounds)
}

#[aoc(day2, part1)]
fn day2_part1(rounds: &[Round]) -> u64 {
    let mut score = 0;

    for round in rounds {
        let mine = round.mine.to_move();
        score += mine.to_score();
        let outcome = match (round.opponent, mine) {
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            _ => Outcome::Draw,
        };
        score += outcome.to_score();
    }

    score
}

#[aoc(day2, part2)]
fn day2_part2(rounds: &[Round]) -> u64 {
    let mut score = 0;

    for round in rounds {
        let outcome = round.mine.to_outcome();
        let mine = match (outcome, round.opponent) {
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
            (Outcome::Draw, opponent) => opponent,
        };
        score += mine.to_score();
        score += outcome.to_score();
    }

    score
}
