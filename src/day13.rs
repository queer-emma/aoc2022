use std::{
    cmp::Ordering,
    iter::Peekable,
    str::{
        Chars,
        FromStr,
    },
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),
}

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
        }
    }

    pub fn parse_number(&mut self) -> Result<i32, ParseError> {
        let mut token = String::new();

        while let Some(c) = self.chars.peek() {
            if c.is_numeric() {
                token.push(*c);
                self.chars.next();
            }
            else {
                break;
            }
        }

        if token.is_empty() {
            Err(ParseError::UnexpectedEnd)
        }
        else {
            // token consists of numeric chars, so should parse correctly
            let number = token.parse().unwrap();
            Ok(number)
        }
    }

    pub fn parse_list(&mut self) -> Result<Vec<Value>, ParseError> {
        match self.chars.next() {
            Some('[') => {}
            Some(c) => return Err(ParseError::UnexpectedCharacter(c)),
            None => {
                return Err(ParseError::UnexpectedEnd);
            }
        }

        let mut values = vec![];

        while let Some(c) = self.chars.peek() {
            match c {
                ']' => {
                    self.chars.next();
                    break;
                }
                _ => {
                    values.push(self.parse_value()?);

                    match self.chars.next() {
                        Some(']') => break,
                        Some(',') => {}
                        Some(c) => return Err(ParseError::UnexpectedCharacter(c)),
                        None => return Err(ParseError::UnexpectedEnd),
                    }
                }
            }
        }

        Ok(values)
    }

    pub fn parse_value(&mut self) -> Result<Value, ParseError> {
        let value = match self.chars.peek() {
            Some('[') => Value::List(self.parse_list()?),
            Some(c) => {
                if c.is_numeric() {
                    Value::Number(self.parse_number()?)
                }
                else {
                    return Err(ParseError::UnexpectedCharacter(*c));
                }
            }
            None => return Err(ParseError::UnexpectedEnd),
        };

        Ok(value)
    }
}

#[derive(Clone, Debug)]
pub struct PacketPair([Packet; 2]);

impl PacketPair {
    pub fn compare(&self) -> CompareResult {
        self.0[0].compare(&self.0[1])
    }

    pub fn is_in_right_order(&self) -> bool {
        match self.compare() {
            CompareResult::RightOrder => true,
            CompareResult::WrongOrder => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet(Vec<Value>);

impl Packet {
    pub fn compare(&self, other: &Self) -> CompareResult {
        compare_lists(&self.0, &other.0).unwrap()
    }
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let values = parser.parse_list()?;
        Ok(Packet(values))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Number(i32),
    List(Vec<Value>),
}

impl Value {
    pub fn compare(&self, other: &Self) -> Option<CompareResult> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => {
                match left.cmp(right) {
                    Ordering::Less => Some(CompareResult::RightOrder),
                    Ordering::Greater => Some(CompareResult::WrongOrder),
                    _ => None,
                }
            }
            (Value::List(left), Value::List(right)) => compare_lists(left, right),
            (Value::Number(left), Value::List(right)) => {
                let left = [Value::Number(*left)];
                compare_lists(&left, right)
            }
            (Value::List(left), Value::Number(right)) => {
                let right = [Value::Number(*right)];
                compare_lists(left, &right)
            }
        }
    }
}

pub fn compare_lists(left: &[Value], right: &[Value]) -> Option<CompareResult> {
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    loop {
        match (left_iter.next(), right_iter.next()) {
            (None, Some(_)) => return Some(CompareResult::RightOrder),
            (Some(_), None) => return Some(CompareResult::WrongOrder),
            (Some(left), Some(right)) => {
                if let Some(result) = left.compare(right) {
                    return Some(result);
                }
            }
            (None, None) => break,
        }
    }

    None
}

#[derive(Copy, Clone, Debug)]
pub enum CompareResult {
    RightOrder,
    WrongOrder,
}

#[aoc_generator(day13)]
fn day13_input(input: &str) -> Vec<PacketPair> {
    let mut lines = input.lines();
    let mut packet_pairs = vec![];

    while let Some(first_line) = lines.next() {
        let second_line = lines.next().unwrap();

        let first_packet = first_line.parse().unwrap();
        let second_packet = second_line.parse().unwrap();

        let packet_pair = PacketPair([first_packet, second_packet]);
        packet_pairs.push(packet_pair);

        // separator
        lines.next();
    }

    packet_pairs
}

#[aoc(day13, part1)]
fn day13_part1(packet_pairs: &[PacketPair]) -> usize {
    let mut sum = 0;
    for (i, pair) in packet_pairs.into_iter().enumerate() {
        if pair.is_in_right_order() {
            sum += i + 1;
        }
    }

    sum
}

#[aoc(day13, part2)]
fn day13_part2(packet_pairs: &[PacketPair]) -> usize {
    let mut packets = vec![];

    for pair in packet_pairs {
        packets.push(pair.0[0].clone());
        packets.push(pair.0[1].clone());
    }

    let divider_packet_2: Packet = "[[2]]".parse().unwrap();
    let divider_packet_6: Packet = "[[6]]".parse().unwrap();
    packets.push(divider_packet_2.clone());
    packets.push(divider_packet_6.clone());

    packets.sort_by(|left, right| {
        match left.compare(right) {
            CompareResult::RightOrder => Ordering::Less,
            CompareResult::WrongOrder => Ordering::Greater,
        }
    });

    let mut divider_packet_indices = [None; 2];

    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider_packet_2 {
            divider_packet_indices[0] = Some(i + 1);
        }
        else if packet == &divider_packet_6 {
            divider_packet_indices[1] = Some(i + 1);
        }
    }

    divider_packet_indices[0].unwrap() * divider_packet_indices[1].unwrap()
}
