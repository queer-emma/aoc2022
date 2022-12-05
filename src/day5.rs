use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_REGEX: Regex = r"move (\d+) from (\d+) to (\d+)".parse().unwrap();
    static ref CRATES_REGEX: Regex = r"(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(    |\[[A-Z]\] )(   |\[[A-Z]\])".parse().unwrap();
}

#[derive(Clone, Copy)]
struct CrateId(char);

impl fmt::Debug for CrateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Clone, Debug, Default)]
struct Stacks([Vec<CrateId>; 9]);

impl Stacks {
    fn top_crates(&self) -> String {
        let mut top_crates = String::new();
        for stack in &self.0 {
            if let Some(crate_id) = stack.last() {
                top_crates.push(crate_id.0);
            }
        }
        top_crates
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn execute_9000(&self, stacks: &mut Stacks) {
        for _ in 0..self.count {
            let crate_id = stacks.0[self.from - 1].pop().unwrap();
            stacks.0[self.to - 1].push(crate_id);
        }
    }

    fn execute_9001(&self, stacks: &mut Stacks) {
        let mut buf = vec![];

        for _ in 0..self.count {
            let crate_id = stacks.0[self.from - 1].pop().unwrap();
            buf.push(crate_id);
        }

        buf.reverse();
        stacks.0[self.to - 1].append(&mut buf);
    }
}

#[derive(Debug)]
struct PuzzleInput {
    stacks: Stacks,
    moves: Vec<Move>,
}

#[aoc_generator(day5)]
fn day5_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();
    let mut stacks: Stacks = Default::default();
    let mut moves = vec![];

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        if let Some(captures) = CRATES_REGEX.captures(line) {
            for i in 0..9 {
                let capture = captures.get(i + 1).unwrap().as_str();
                let crate_id = capture[1..].chars().next().unwrap();
                if crate_id != ' ' {
                    stacks.0[i].push(CrateId(crate_id));
                }
            }
        }
    }

    for i in 0..9 {
        stacks.0[i].reverse();
    }

    while let Some(line) = lines.next() {
        let captures = MOVE_REGEX.captures(line).unwrap();

        moves.push(Move {
            count: captures.get(1).unwrap().as_str().parse().unwrap(),
            from: captures.get(2).unwrap().as_str().parse().unwrap(),
            to: captures.get(3).unwrap().as_str().parse().unwrap(),
        })
    }

    PuzzleInput { stacks, moves }
}

#[aoc(day5, part1)]
fn day5_part1(input: &PuzzleInput) -> String {
    let mut stacks = input.stacks.clone();

    for mov in &input.moves {
        mov.execute_9000(&mut stacks);
    }

    stacks.top_crates()
}

#[aoc(day5, part2)]
fn day5_part2(input: &PuzzleInput) -> String {
    let mut stacks = input.stacks.clone();

    for mov in &input.moves {
        mov.execute_9001(&mut stacks);
    }

    stacks.top_crates()
}
