use nalgebra::Vector2;

pub struct HeightMap {
    elevation_data: Vec<u8>,
    width: i32,
    height: i32,
    start_position: Vector2<i32>,
    best_signal: Vector2<i32>,
}

impl HeightMap {
    pub fn get_height(&self, position: Vector2<i32>) -> i32 {
        assert!(position.x >= 0 && position.x < self.width);
        assert!(position.y >= 0 && position.y < self.height);
        self.elevation_data[(position.y * self.width + position.x) as usize] as i32
    }

    pub fn neighbors(&self, position: Vector2<i32>) -> Vec<Vector2<i32>> {
        let mut neighbors = vec![];
        let height = self.get_height(position);

        let mut consider_neighbor = |dx, dy| {
            let neighbor = position + Vector2::new(dx, dy);
            if height - self.get_height(neighbor) <= 1 {
                neighbors.push(neighbor)
            }
        };

        if position.x > 0 {
            consider_neighbor(-1, 0);
        }
        if position.x < self.width - 1 {
            consider_neighbor(1, 0);
        }
        if position.y > 0 {
            consider_neighbor(0, -1);
        }
        if position.y < self.height - 1 {
            consider_neighbor(0, 1);
        }

        neighbors
    }

    pub fn shortest_path_to_best_signal(&self) -> Vec<Vector2<i32>> {
        // note: we search from destination to start, so that we can use the same
        // neighbor function for part b.

        let mut path = pathfinding::prelude::bfs(
            &self.best_signal,
            |position| self.neighbors(*position),
            |position| position == &self.start_position,
        )
        .expect("path");

        path.reverse();

        path
    }

    pub fn shortest_path_from_lowest_elevation(&self) -> Vec<Vector2<i32>> {
        let mut path = pathfinding::prelude::bfs(
            &self.best_signal,
            |position| self.neighbors(*position),
            |position| self.get_height(*position) == 0,
        )
        .expect("path");

        path.reverse();

        path
    }
}

pub struct Path(Vec<Vector2<u32>>);

#[aoc_generator(day12)]
fn day12_input(input: &str) -> HeightMap {
    let mut width = 0;
    let mut height = 0;
    let mut start_position = Vector2::zeros();
    let mut best_signal = Vector2::zeros();
    let mut elevation_data = vec![];

    for (y, line) in input.lines().enumerate() {
        height += 1;
        if width == 0 {
            width = line.len() as i32;
        }

        for (x, mut c) in line.chars().enumerate() {
            if c == 'S' {
                start_position = Vector2::new(x as i32, y as i32);
                c = 'a';
            }
            else if c == 'E' {
                best_signal = Vector2::new(x as i32, y as i32);
                c = 'z';
            }
            let elevation: u8 = (u32::from(c) - u32::from('a')).try_into().unwrap();
            elevation_data.push(elevation);
        }
    }

    HeightMap {
        elevation_data,
        width,
        height,
        start_position,
        best_signal,
    }
}

#[aoc(day12, part1)]
fn day12_part1(height_map: &HeightMap) -> usize {
    let path = height_map.shortest_path_to_best_signal();

    // note: the path contains the start and end position, so the number of steps is
    // exactly one less than the number of nodes visited.
    path.len() - 1
}

#[aoc(day12, part2)]
fn day12_part2(height_map: &HeightMap) -> usize {
    let path = height_map.shortest_path_from_lowest_elevation();
    path.len() - 1
}
