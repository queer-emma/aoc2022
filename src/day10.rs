use std::str::FromStr;

use thiserror::Error;

pub struct FrameBuffer {
    data: [bool; Self::NUM_PIXELS],
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            data: [false; Self::NUM_PIXELS],
        }
    }
}

impl FrameBuffer {
    const NUM_PIXELS_PER_ROW: usize = 40;
    const NUM_ROWS: usize = 6;
    const NUM_PIXELS: usize = Self::NUM_PIXELS_PER_ROW * Self::NUM_ROWS;

    pub fn send_data(&mut self, cycle: u64, sprite: i64) {
        let line = (cycle as i64 - 1) / Self::NUM_PIXELS_PER_ROW as i64;
        let col = (cycle as i64 - 1) % Self::NUM_PIXELS_PER_ROW as i64;

        if line >= 0 && line < 6 && col >= 0 && col < 40 {
            if sprite - 1 <= col && sprite + 1 >= col {
                self.data[line as usize * Self::NUM_PIXELS_PER_ROW + col as usize] = true;
            }
        }
    }

    pub fn print(&self) {
        for line in 0..Self::NUM_ROWS {
            for col in 0..Self::NUM_PIXELS_PER_ROW {
                if self.data[line * Self::NUM_PIXELS_PER_ROW + col] {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

pub struct Cpu {
    x_register: i64,
    signal: Vec<i64>,
    cycle: u64,
    frame_buffer: FrameBuffer,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x_register: 1,
            signal: vec![],
            cycle: 1,
            frame_buffer: Default::default(),
        }
    }
}

impl Cpu {
    pub fn run_instruction(&mut self, instruction: Instruction) {
        let cycle_after_execution = self.cycle + instruction.cycles();

        for cycle in self.cycle..cycle_after_execution {
            if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                let signal_strength = cycle as i64 * self.x_register;
                //println!("cycle={}, x={}, signal={}", cycle, self.x_register,
                // signal_strength);
                self.signal.push(signal_strength);
            }

            self.frame_buffer.send_data(cycle, self.x_register);
        }

        //println!("cycle={}, x={}, instruction={:?}", cycle_after_execution,
        // self.x_register, instruction);
        instruction.execute(&mut self.x_register);
        self.cycle = cycle_after_execution;
    }

    pub fn run_program(&mut self, program: &[Instruction]) {
        for instruction in program {
            self.run_instruction(*instruction);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Add(i64),
    Noop,
}

impl Instruction {
    pub fn cycles(&self) -> u64 {
        match self {
            Self::Add(_) => 2,
            Self::Noop => 1,
        }
    }

    pub fn execute(&self, x_register: &mut i64) {
        match self {
            Instruction::Add(x) => *x_register += x,
            Instruction::Noop => {}
        }
    }
}

#[derive(Debug, Error)]
#[error("failed to parse instruction: {0}")]
pub struct InstructionParseError(String);

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || InstructionParseError(s.to_owned());

        if s == "noop" {
            Ok(Self::Noop)
        }
        else {
            let operand: i64 = s
                .strip_prefix("addx ")
                .ok_or_else(err)?
                .parse()
                .map_err(|_| err())?;
            Ok(Self::Add(operand))
        }
    }
}

#[aoc_generator(day10)]
fn day10_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn day10_part1(program: &[Instruction]) -> i64 {
    let mut cpu = Cpu::default();
    cpu.run_program(program);
    cpu.signal.iter().sum()
}

#[aoc(day10, part2)]
fn day10_part2(program: &[Instruction]) -> &'static str {
    let mut cpu = Cpu::default();
    cpu.run_program(program);
    cpu.frame_buffer.print();
    "read from framebuffer"
}
