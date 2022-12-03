use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> u64 {
        if self.0.is_lowercase() {
            u64::from(self.0) - u64::from('a') + 1
        }
        else {
            u64::from(self.0) - u64::from('A') + 27
        }
    }
}

struct Rucksack {
    first: HashSet<Item>,
    second: HashSet<Item>,
}

impl Rucksack {
    pub fn in_both(&self) -> Item {
        let mut intersection = self.first.intersection(&self.second);
        let item = intersection
            .next()
            .unwrap_or_else(|| panic!("no item in both compartments"));
        *item
    }

    pub fn all_items(&self) -> HashSet<Item> {
        let mut all = HashSet::new();
        all.extend(&self.first);
        all.extend(&self.second);
        all
    }
}

fn find_badge(rucksacks: [&Rucksack; 3]) -> Item {
    let items = rucksacks.map(|rucksack| rucksack.all_items());
    let intersection = items[0]
        .intersection(&items[1])
        .copied()
        .collect::<HashSet<_>>();
    let intersection = intersection.intersection(&items[2]).collect::<Vec<_>>();

    if intersection.len() != 1 {
        panic!("bag intersection: {}", intersection.len());
    }

    **intersection.first().unwrap()
}

#[aoc_generator(day3)]
fn day3_input(input: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];

    for line in input.lines() {
        let n = line.len() / 2;
        let first = line[..n].chars().map(Item).collect();
        let second = line[n..].chars().map(Item).collect();
        rucksacks.push(Rucksack { first, second })
    }

    rucksacks
}

#[aoc(day3, part1)]
fn day3_part1(rucksacks: &[Rucksack]) -> u64 {
    rucksacks
        .into_iter()
        .map(|rucksack| rucksack.in_both().priority())
        .sum()
}

#[aoc(day3, part2)]
fn day3_part2(rucksacks: &[Rucksack]) -> u64 {
    let mut rucksack_iter = rucksacks.into_iter();
    let mut priorities = 0;

    loop {
        let Some(first) = rucksack_iter.next() else {break};
        let group = [
            first,
            rucksack_iter.next().unwrap(),
            rucksack_iter.next().unwrap(),
        ];
        let badge = find_badge(group);
        priorities += badge.priority();
    }

    priorities
}
