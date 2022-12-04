use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_ASSIGNMENT: Regex = r"(\d+)-(\d+),(\d+)-(\d+)".parse().unwrap();
}

#[derive(Copy, Clone, Debug)]
pub struct Assignment {
    first: Range,
    second: Range,
}

impl Assignment {
    pub fn fully_contains_other(&self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    pub fn overlap_at_all(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Range {
    first: u64,
    last: u64,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        other.first >= self.first && other.last <= self.last
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.first >= other.first && self.first <= other.last)
            || (self.last >= other.first && self.last <= other.last)
            || self.fully_contains(other)
            || other.fully_contains(self)
    }
}

#[aoc_generator(day4)]
fn day4_input(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|line| {
            let captures = REGEX_ASSIGNMENT.captures(line).unwrap();
            let first = Range {
                first: captures.get(1).unwrap().as_str().parse().unwrap(),
                last: captures.get(2).unwrap().as_str().parse().unwrap(),
            };
            let second = Range {
                first: captures.get(3).unwrap().as_str().parse().unwrap(),
                last: captures.get(4).unwrap().as_str().parse().unwrap(),
            };

            Assignment { first, second }
        })
        .collect()
}

#[aoc(day4, part1)]
fn day4_part1(assignments: &[Assignment]) -> usize {
    assignments
        .into_iter()
        .filter(|assignment| assignment.fully_contains_other())
        .count()
}

#[aoc(day4, part2)]
fn day4_part2(assignments: &[Assignment]) -> usize {
    assignments
        .into_iter()
        .filter(|assignment| assignment.overlap_at_all())
        .count()
}
