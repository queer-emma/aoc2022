use std::collections::HashSet;

use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;

lazy_static! {
    static ref SENSOR_SIGNAL_REGEX: Regex =
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            .parse()
            .unwrap();
}

pub fn manhattan_distance(a: Vector2<i32>, b: Vector2<i32>) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub struct Sensor {
    position: Vector2<i32>,
    closest_beacon: Vector2<i32>,
}

impl Sensor {
    pub fn beacon_distance(&self) -> i32 {
        manhattan_distance(self.position, self.closest_beacon)
    }
}

#[aoc_generator(day15)]
fn day15_input(input: &str) -> Vec<Sensor> {
    let mut sensor_signals = vec![];
    for line in input.lines() {
        let captures = SENSOR_SIGNAL_REGEX.captures(line).unwrap();

        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let y = captures.get(2).unwrap().as_str().parse().unwrap();
        let position = Vector2::new(x, y);

        let x = captures.get(3).unwrap().as_str().parse().unwrap();
        let y = captures.get(4).unwrap().as_str().parse().unwrap();
        let closest_beacon = Vector2::new(x, y);

        sensor_signals.push(Sensor {
            position,
            closest_beacon,
        })
    }

    sensor_signals
}

pub struct Sensors<'a> {
    sensors: &'a [Sensor],
    beacon_positions: HashSet<Vector2<i32>>,
}

impl<'a> Sensors<'a> {
    pub fn new(sensors: &'a [Sensor]) -> Self {
        let mut beacon_positions = HashSet::new();

        // put all beacons into a hashset so we can exclude these positions
        for sensor in sensors {
            beacon_positions.insert(sensor.closest_beacon);
        }

        Self {
            sensors,
            beacon_positions,
        }
    }

    pub fn covered_positions_for_row(&self, y: i32) -> HashSet<Vector2<i32>> {
        let mut covered_positions = HashSet::new();

        for sensor in self.sensors {
            // given the beacon distance we can compute the x range where beacons can't be.
            let beacon_distance = sensor.beacon_distance();
            let y_distance = (sensor.position.y - y).abs();
            let x_distance = beacon_distance - y_distance;

            // there can't be another beacon at the same distance, so x_distance must be >=
            // 0
            if x_distance >= 0 {
                //println!("sensor.position = {:?}", sensor.position);
                //println!("sensor.closest_beacon = {:?}", sensor.closest_beacon);
                //println!("beacon_distance = {}", beacon_distance);
                //println!("y_distance = {}", y_distance);
                //println!("x_distance = {}", x_distance);

                for x in sensor.position.x - x_distance..=sensor.position.x + x_distance {
                    let position = Vector2::new(x, y);

                    if !self.beacon_positions.contains(&position) {
                        //let d = manhattan_distance(position, sensor.position);
                        //println!("  {:?}, d={}", position, d);
                        //assert!(d <= beacon_distance);
                        covered_positions.insert(position);
                    }
                }

                //println!("");
            }
        }

        covered_positions
    }

    pub fn find_distress_signal(&self, coord_range: i32) -> Vector2<i32> {
        for y in 0..=coord_range {
            let covered_positions = self.covered_positions_for_row(y);
            println!("{}: covered_positions={}", y, covered_positions.len());

            for x in 0..=coord_range {
                let position = Vector2::new(x, y);
                if !covered_positions.contains(&position)
                    && !self.beacon_positions.contains(&position)
                {
                    println!("found distress signal: {:?}", position);
                    return position;
                }
            }
        }

        panic!("no distress signal found")
    }
}

#[aoc(day15, part1)]
fn day15_part1(sensors: &[Sensor]) -> usize {
    let sensors = Sensors::new(sensors);
    let covered_positions = sensors.covered_positions_for_row(2000000);
    covered_positions.len()
}

#[aoc(day15, part2)]
fn day15_part2(sensors: &[Sensor]) -> i32 {
    let sensors = Sensors::new(sensors);
    let distress_signal = sensors.find_distress_signal(4000000);

    distress_signal.x * 4000000 + distress_signal.y
}
