use std::collections::BTreeMap;

use itertools::Itertools;
use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Rock,
    Sand,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SimulationOutcome {
    SandFellIntoVoid,
    SandRests,
    SourceBlocked,
}

#[derive(Clone, Debug)]
pub struct Sandbox {
    tiles: BTreeMap<(i32, i32), Tile>,
    source: Vector2<i32>,
    min: Vector2<i32>,
    max: Vector2<i32>,
    floor: bool,
}

impl Sandbox {
    pub fn from_rock_paths(paths: &RockPaths, floor: bool) -> Self {
        let mut tiles = BTreeMap::new();
        let source = Vector2::new(500, 0);
        let mut min: Vector2<i32> = source;
        let mut max: Vector2<i32> = source;

        let mut add_tile = |x, y| {
            // insert tile
            tiles.insert((x, y), Tile::Rock);

            // track bounding box of sandbox
            if x < min.x {
                min.x = x;
            }
            if y < min.y {
                min.y = y;
            }
            if x > max.x {
                max.x = x;
            }
            if y > max.y {
                max.y = y;
            }
        };

        for path in &paths.0 {
            for (a, b) in path.into_iter().tuple_windows() {
                if a.x < b.x {
                    // horizontal
                    for x in a.x..=b.x {
                        add_tile(x, a.y);
                    }
                }
                else if a.x > b.x {
                    // horizontal
                    for x in b.x..=a.x {
                        add_tile(x, a.y);
                    }
                }
                else if a.y < b.y {
                    // vertical
                    for y in a.y..=b.y {
                        add_tile(a.x, y);
                    }
                }
                else if a.y > b.y {
                    // vertical
                    for y in b.y..=a.y {
                        add_tile(a.x, y);
                    }
                }
                else {
                    panic!(
                        "start and end position of rock are the same: {:?}, {:?}",
                        a, b
                    );
                }
            }
        }

        Sandbox {
            tiles,
            source,
            min,
            max,
            floor,
        }
    }

    pub fn get_tile(&self, position: Vector2<i32>) -> Tile {
        self.tiles
            .get(&(position.x, position.y))
            .copied()
            .unwrap_or(Tile::Empty)
    }

    pub fn set_tile(&mut self, position: Vector2<i32>, tile: Tile) {
        self.tiles.insert((position.x, position.y), tile);
    }

    pub fn print(&self) {
        for y in self.min.y - 2..=self.max.y + 2 {
            print!("{:>3} ", y);

            for x in self.min.x - 2..=self.max.x + 2 {
                match self.get_tile(Vector2::new(x, y)) {
                    Tile::Empty => {
                        if x == self.source.x && y == self.source.y {
                            print!("+")
                        }
                        else if self.floor && y == self.max.y + 2 {
                            print!("#")
                        }
                        else {
                            print!(".")
                        }
                    }
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                }
            }
            println!("");
        }
        println!("");
    }

    /// returns whether the sand fell into the void
    pub fn simulate_sand_particle(&mut self) -> SimulationOutcome {
        if self.get_tile(self.source) == Tile::Sand {
            return SimulationOutcome::SourceBlocked;
        }

        let mut sand_position = self.source;

        loop {
            let down = sand_position + Vector2::new(0, 1);
            let down_left = sand_position + Vector2::new(-1, 1);
            let down_right = sand_position + Vector2::new(1, 1);

            if self.get_tile(down) == Tile::Empty {
                sand_position = down;
            }
            else if self.get_tile(down_left) == Tile::Empty {
                sand_position = down_left;
            }
            else if self.get_tile(down_right) == Tile::Empty {
                sand_position = down_right;
            }
            else {
                // sand can't move anymore
                self.set_tile(sand_position, Tile::Sand);
                return SimulationOutcome::SandRests;
            }

            if self.floor {
                // part 2
                if sand_position.y == self.max.y + 1 {
                    // sand can't move anymore
                    self.set_tile(sand_position, Tile::Sand);
                    return SimulationOutcome::SandRests;
                }
            }
            else {
                // part 1
                if sand_position.y > self.max.y {
                    return SimulationOutcome::SandFellIntoVoid;
                }
            }
        }
    }
}

pub struct RockPaths(Vec<Vec<Vector2<i32>>>);

#[aoc_generator(day14)]
fn day14_input(input: &str) -> RockPaths {
    let mut rock_paths = vec![];

    for line in input.lines() {
        let mut path = vec![];

        for point in line.split(" -> ") {
            let (x, y) = point.split_once(",").unwrap();
            let point: Vector2<i32> = Vector2::new(x.parse().unwrap(), y.parse().unwrap());
            path.push(point);
        }

        rock_paths.push(path);
    }

    RockPaths(rock_paths)
}

#[aoc(day14, part1)]
fn day14_part1(rock_paths: &RockPaths) -> usize {
    let mut sandbox = Sandbox::from_rock_paths(rock_paths, false);
    let mut num_sand = 0;

    sandbox.print();

    while sandbox.simulate_sand_particle() != SimulationOutcome::SandFellIntoVoid {
        num_sand += 1;

        println!("sand particles: {}", num_sand);
        sandbox.print();
    }

    num_sand
}

#[aoc(day14, part2)]
fn day14_part2(rock_paths: &RockPaths) -> usize {
    let mut sandbox = Sandbox::from_rock_paths(rock_paths, true);
    let mut num_sand = 0;

    sandbox.print();

    while sandbox.simulate_sand_particle() != SimulationOutcome::SourceBlocked {
        num_sand += 1;
    }

    println!("sand particles: {}", num_sand);
    sandbox.print();

    num_sand
}
