use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_traits::Zero;
use regex::Regex;

pub type MonkeyId = u8;

lazy_static! {
    static ref MONKEY_REGEX: Regex = r"^Monkey (\d+):$".parse().unwrap();
    static ref OPERATION_REGEX: Regex = r"^  Operation: new = ([\w\d]+) ([\*\+]) ([\w\d]+)$"
        .parse()
        .unwrap();
}

#[derive(Clone, Debug)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
    dispatch: Vec<Vec<BigInt>>,
    divisor: BigInt,
}

impl Monkeys {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        let mut dispatch = Vec::with_capacity(monkeys.len());
        dispatch.resize_with(monkeys.len(), Default::default);

        let mut divisor = 1.into();
        for monkey in &monkeys {
            divisor *= &monkey.test.divisible_by;
        }

        Monkeys {
            monkeys,
            dispatch,
            divisor,
        }
    }

    pub fn round(&mut self, reduce_worry_level: bool) {
        for i in 0..self.monkeys.len() {
            {
                let monkey = &mut self.monkeys[i];

                // check on monkey and decide where items go
                for worry_level in &monkey.items {
                    let mut worry_level = monkey.operation.eval(worry_level);
                    if reduce_worry_level {
                        worry_level /= 3;
                    }
                    else {
                        worry_level %= &self.divisor;
                    }
                    let next_monkey = monkey.test.eval(&worry_level);
                    self.dispatch[next_monkey as usize].push(worry_level);
                }
                monkey.inspect_count += monkey.items.len();
                monkey.items.clear();

                // send items to new monkeys
                for (worry_levels, monkey) in self.dispatch.iter_mut().zip(&mut self.monkeys) {
                    for worry_level in worry_levels.drain(..) {
                        monkey.items.push(worry_level);
                    }
                }
            }
        }
    }

    pub fn monkey_business(&self) -> usize {
        let mut inspect_counts = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspect_count)
            .collect::<Vec<_>>();
        inspect_counts.sort();
        inspect_counts.reverse();
        inspect_counts[0] * inspect_counts[1]
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<BigInt>,
    operation: WorryLevelOperation,
    test: Test,
    inspect_count: usize,
}

#[derive(Clone, Debug)]
pub struct WorryLevelOperation {
    left: Operand,
    right: Operand,
    operation: Operation,
}

impl WorryLevelOperation {
    pub fn eval(&self, old_value: &BigInt) -> BigInt {
        let left = self.left.eval(old_value);
        let right = self.right.eval(old_value);
        self.operation.eval(left, right)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Add,
    Mul,
}

impl Operation {
    pub fn eval(&self, left: BigInt, right: BigInt) -> BigInt {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operand {
    Old,
    Constant(BigInt),
}

impl Operand {
    pub fn eval(&self, old_value: &BigInt) -> BigInt {
        match self {
            Self::Old => old_value.clone(),
            Self::Constant(constant) => constant.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Test {
    divisible_by: BigInt,
    true_monkey: MonkeyId,
    false_monkey: MonkeyId,
}

impl Test {
    pub fn eval(&self, value: &BigInt) -> MonkeyId {
        if (value % &self.divisible_by).is_zero() {
            self.true_monkey
        }
        else {
            self.false_monkey
        }
    }
}

#[aoc_generator(day11)]
fn day11_input(input: &str) -> Monkeys {
    fn parse_operand(s: &str) -> Operand {
        if s == "old" {
            Operand::Old
        }
        else {
            Operand::Constant(s.parse().unwrap())
        }
    }
    let mut lines = input.lines();
    let mut monkeys = vec![];

    while let Some(line) = lines.next() {
        assert!(MONKEY_REGEX.is_match(line));

        let items = lines
            .next()
            .expect("line with starting items")
            .strip_prefix("  Starting items: ")
            .expect("starting items prefix")
            .split(", ")
            .map(|s| {
                s.parse::<BigInt>()
                    .expect("starting item worry level as number")
            })
            .collect();

        let captures = OPERATION_REGEX.captures(lines.next().unwrap()).unwrap();
        let left = parse_operand(captures.get(1).unwrap().as_str());
        let operation = match captures.get(2).unwrap().as_str() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            op_str => panic!("unknown operation: {}", op_str),
        };
        let right = parse_operand(captures.get(3).unwrap().as_str());
        let operation = WorryLevelOperation {
            left,
            right,
            operation,
        };

        let divisible_by = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let test = Test {
            divisible_by,
            true_monkey,
            false_monkey,
        };

        let spacing = lines.next();
        assert!(spacing == None || spacing.unwrap() == "");

        let monkey = Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        };
        monkeys.push(monkey)
    }

    Monkeys::new(monkeys)
}

fn print_monkey_business(monkeys: &Monkeys) {
    for (i, monkey) in monkeys.monkeys.iter().enumerate() {
        println!(
            "monkey {} inspected items {} times.",
            i, monkey.inspect_count
        );
    }
}

#[aoc(day11, part1)]
fn day11_part1(monkeys: &Monkeys) -> usize {
    let mut monkeys = monkeys.clone();

    for _ in 0..20 {
        monkeys.round(true);
    }

    print_monkey_business(&monkeys);

    monkeys.monkey_business()
}

#[aoc(day11, part2)]
fn day11_part2(monkeys: &Monkeys) -> usize {
    let mut monkeys = monkeys.clone();

    for round in 1..=10000 {
        monkeys.round(false);
        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("== after round {} ==", round);
            print_monkey_business(&monkeys);
        }
    }

    monkeys.monkey_business()
}
