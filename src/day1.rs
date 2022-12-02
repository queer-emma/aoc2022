use std::num::ParseIntError;

#[aoc_generator(day1)]
fn day1_input(input: &str) -> Vec<Vec<u32>> {
    let mut calories = Vec::new();
    let mut buf = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            calories.push(buf.drain(..).collect());
        }
        else {
            let current = line.parse().unwrap();
            buf.push(current);
        }
    }

    calories
}

#[aoc(day1, part1)]
fn day1_part1(calories: &[Vec<u32>]) -> u32 {
    calories.iter().map(|c| c.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn day1_part2(calories: &[Vec<u32>]) -> u32 {
    let mut calories = calories
        .iter()
        .map(|c| c.iter().sum())
        .collect::<Vec<u32>>();

    calories.sort_by(|a, b| a.cmp(b).reverse());

    calories[0..3].iter().sum()
}
