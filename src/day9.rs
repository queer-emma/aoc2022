use std::{
    collections::HashSet,
    str::FromStr,
};

use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;
use thiserror::Error;

lazy_static! {
    pub static ref MOVEMENT_REGEX: Regex = r"([UDLR]) (\d+)".parse().unwrap();
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Movement {
    direction: Direction,
    count: u64,
}

#[derive(Debug, Error)]
#[error("failed to parse movement: {0}")]
pub struct MovementParseError(String);

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || MovementParseError(s.to_owned());
        let captures = MOVEMENT_REGEX.captures(s).ok_or_else(err)?;
        let direction = match captures.get(1).ok_or_else(err)?.as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(err()),
        };
        let count = captures
            .get(2)
            .ok_or_else(err)?
            .as_str()
            .parse()
            .map_err(|_| err())?;
        Ok(Movement { direction, count })
    }
}

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Vector2<i32>>,
    tail_positions: HashSet<Vector2<i32>>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut knots = Vec::with_capacity(length);
        knots.resize_with(length, Default::default);

        let mut tail_positions = HashSet::new();
        tail_positions.insert(Vector2::new(0, 0));

        Self {
            knots,
            tail_positions,
        }
    }

    pub fn apply(&mut self, movement: Movement) {
        //println!("== {:?} ==", movement);

        for _ in 0..movement.count {
            self.move_head(movement.direction);
            //self.print();
        }
    }

    fn move_head(&mut self, direction: Direction) {
        // move head
        let d_head = match direction {
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        };
        self.knots[0] += d_head;

        // move knots
        for i in 1..self.knots.len() {
            let d = self.knots[i - 1] - self.knots[i];

            if d.x.abs() > 1 || d.y.abs() > 1 {
                self.knots[i].x += d.x.signum();
                self.knots[i].y += d.y.signum();
            }
        }

        // track tail
        self.tail_positions.insert(self.knots[self.knots.len() - 1]);
    }

    pub fn print(&self) {
        for y in (0..5).rev() {
            for x in 0..6 {
                let x = Vector2::new(x, y);

                if self.knots[0] == x {
                    print!("H");
                }
                else {
                    let mut knot_printed = false;

                    for i in 1..self.knots.len() {
                        if x == self.knots[i] {
                            print!("{}", i);
                            knot_printed = true;
                            break;
                        }
                    }

                    if !knot_printed {
                        if x == Vector2::zeros() {
                            print!("s");
                        }
                        else {
                            print!(".");
                        }
                    }
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn print_tail(&self) {
        for y in (0..5).rev() {
            for x in 0..6 {
                let x = Vector2::<i32>::new(x, y);
                if x == Vector2::zeros() {
                    print!("s");
                }
                else if self.tail_positions.contains(&x) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn num_tail_positions(&self) -> usize {
        self.tail_positions.len()
    }
}

fn simulate_rope(length: usize, movements: &[Movement]) -> usize {
    let mut rope = Rope::new(length);

    for movement in movements {
        rope.apply(*movement);
    }

    rope.num_tail_positions()
}

#[aoc_generator(day9)]
fn day9_input(input: &str) -> Vec<Movement> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn day9_part1(movements: &[Movement]) -> usize {
    simulate_rope(2, movements)
}

#[aoc(day9, part2)]
fn day9_part2(movements: &[Movement]) -> usize {
    simulate_rope(10, movements)
}
