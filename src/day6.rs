use std::collections::{
    hash_map::Entry,
    HashMap,
};

#[derive(Debug)]
struct SignalBuffer {
    buf: HashMap<char, usize>,
    unique_count: usize,
}

impl Default for SignalBuffer {
    fn default() -> Self {
        Self {
            buf: HashMap::new(),
            unique_count: 0,
        }
    }
}

impl SignalBuffer {
    pub fn remove(&mut self, c: char) {
        let entry = self.buf.get_mut(&c).unwrap();
        *entry -= 1;
        if *entry == 0 {
            self.unique_count -= 1;
        }
    }

    pub fn insert(&mut self, c: char) {
        match self.buf.entry(c) {
            Entry::Occupied(mut occupied) => {
                let value = occupied.get_mut();
                if *value == 0 {
                    self.unique_count += 1;
                }
                *value += 1
            }
            Entry::Vacant(vacant) => {
                vacant.insert(1);
                self.unique_count += 1;
            }
        }
    }

    pub fn unique_count(&self) -> usize {
        self.unique_count
    }
}

fn find_start_marker(length: usize, signal: &[char]) -> usize {
    let mut buf = SignalBuffer::default();

    for add_index in 0..signal.len() {
        if let Some(remove_index) = add_index.checked_sub(length) {
            buf.remove(signal[remove_index]);
        }
        buf.insert(signal[add_index]);
        if buf.unique_count() == length {
            return add_index + 1;
        }
    }

    panic!("no start marker found");
}

#[aoc_generator(day6)]
fn day6_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day6, part1)]
fn day6_part1(signal: &[char]) -> usize {
    find_start_marker(4, signal)
}

#[aoc(day6, part2)]
fn day6_part2(signal: &[char]) -> usize {
    find_start_marker(14, signal)
}
