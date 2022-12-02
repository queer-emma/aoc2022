use thiserror::Error;

#[derive(Clone, Copy, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn to_score(&self) -> u64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
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
    pub fn to_play(&self) -> Play {
        match self {
            Column2::X => Play::Rock,
            Column2::Y => Play::Paper,
            Column2::Z => Play::Scissors,
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
    opponent: Play,
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
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
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
        let mine = round.mine.to_play();
        score += mine.to_score();
        let outcome = match (round.opponent, mine) {
            (Play::Rock, Play::Scissors) => Outcome::Lose,
            (Play::Rock, Play::Paper) => Outcome::Win,
            (Play::Paper, Play::Rock) => Outcome::Lose,
            (Play::Paper, Play::Scissors) => Outcome::Win,
            (Play::Scissors, Play::Rock) => Outcome::Win,
            (Play::Scissors, Play::Paper) => Outcome::Lose,
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
            (Outcome::Win, Play::Rock) => Play::Paper,
            (Outcome::Win, Play::Paper) => Play::Scissors,
            (Outcome::Win, Play::Scissors) => Play::Rock,
            (Outcome::Lose, Play::Rock) => Play::Scissors,
            (Outcome::Lose, Play::Paper) => Play::Rock,
            (Outcome::Lose, Play::Scissors) => Play::Paper,
            (Outcome::Draw, opponent) => opponent,
        };
        score += mine.to_score();
        score += outcome.to_score();
    }

    score
}
