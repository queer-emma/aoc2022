use std::collections::HashSet;

pub struct Grid {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.data[y][x] as i8
    }

    pub fn visible_trees(&self) -> usize {
        let mut visible = HashSet::new();

        for y in 0..self.height {
            let mut max_height = -1;
            for x in 0..self.width {
                let height = self.get(x, y);
                if height > max_height {
                    visible.insert((x, y));
                    max_height = height;
                }
            }
        }

        for y in 0..self.height {
            let mut max_height = -1;
            for x in (0..self.width).rev() {
                let height = self.get(x, y);
                if height > max_height {
                    visible.insert((x, y));
                    max_height = height;
                }
            }
        }

        for x in 0..self.width {
            let mut max_height = -1;
            for y in 0..self.height {
                let height = self.get(x, y);
                if height > max_height {
                    visible.insert((x, y));
                    max_height = height;
                }
            }
        }

        for x in 0..self.width {
            let mut max_height = -1;
            for y in (0..self.height).rev() {
                let height = self.get(x, y);
                if height > max_height {
                    visible.insert((x, y));
                    max_height = height;
                }
            }
        }

        visible.len()
    }

    pub fn scenic_score(&self, x0: usize, y0: usize) -> usize {
        let h0 = self.get(x0, y0);

        let mut west = 0;
        for x in (0..x0).rev() {
            let h = self.get(x, y0);
            west += 1;
            if h >= h0 {
                break;
            }
        }

        let mut east = 0;
        for x in (x0 + 1)..self.width {
            let h = self.get(x, y0);
            east += 1;
            if h >= h0 {
                break;
            }
        }

        let mut north = 0;
        for y in (0..y0).rev() {
            let h = self.get(x0, y);
            north += 1;
            if h >= h0 {
                break;
            }
        }

        let mut south = 0;
        for y in (y0 + 1)..self.height {
            let h = self.get(x0, y);
            south += 1;
            if h >= h0 {
                break;
            }
        }

        west * east * north * south
    }

    pub fn best_scenic_score(&self) -> usize {
        let mut best = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let score = self.scenic_score(x, y);

                if let Some((best_at, best_score)) = &mut best {
                    if score > *best_score {
                        *best_at = (x, y);
                        *best_score = score;
                    }
                }
                else {
                    best = Some(((x, y), score));
                }
            }
        }

        best.unwrap().1
    }
}

#[aoc_generator(day8)]
fn day8_input(input: &str) -> Grid {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let width = data[0].len();
    let height = data.len();

    Grid {
        data,
        width,
        height,
    }
}

#[aoc(day8, part1)]
fn day8_part1(grid: &Grid) -> usize {
    grid.visible_trees()
}

#[aoc(day8, part2)]
fn day8_part2(grid: &Grid) -> usize {
    grid.best_scenic_score()
}
