use std::collections::HashSet;

use lazy_static::lazy_static;
use nalgebra::Vector2;
use rangemap::RangeInclusiveSet;
use regex::Regex;

lazy_static! {
    static ref SENSOR_SIGNAL_REGEX: Regex =
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            .parse()
            .unwrap();
}

pub fn manhattan_distance(a: Vector2<i64>, b: Vector2<i64>) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub struct Sensor {
    position: Vector2<i64>,
    closest_beacon: Vector2<i64>,
}

impl Sensor {
    pub fn beacon_distance(&self) -> i64 {
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
    beacon_positions: HashSet<Vector2<i64>>,
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

    pub fn covered_positions_for_row(&self, y: i64) -> RangeInclusiveSet<i64> {
        let mut covered_positions = RangeInclusiveSet::new();

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

                let lower = sensor.position.x - x_distance;
                let higher = sensor.position.x + x_distance;
                covered_positions.insert(lower..=higher);

                //println!("");
            }
        }

        covered_positions
    }

    pub fn num_covered_positions_for_row(&self, y: i64) -> i64 {
        let covered_positions = self.covered_positions_for_row(y);
        let mut n = 0;

        for range in covered_positions.iter() {
            n += range.end() - range.start();
        }

        n
    }

    pub fn find_distress_signal(&self, max_xy: i64) -> Vector2<i64> {
        for y in 0..=max_xy {
            let covered_positions = self.covered_positions_for_row(y);
            for gap in covered_positions.gaps(&(0..=max_xy)) {
                assert!(gap.start() == gap.end());
                return Vector2::new(*gap.start(), y);
            }
        }

        panic!("no distress signal found")
    }
}

#[aoc(day15, part1)]
fn day15_part1(sensors: &[Sensor]) -> i64 {
    let sensors = Sensors::new(sensors);
    //sensors.num_covered_positions_for_row(20)
    sensors.num_covered_positions_for_row(2000000)
}

#[aoc(day15, part2)]
fn day15_part2(sensors: &[Sensor]) -> i64 {
    let sensors = Sensors::new(sensors);
    //let distress_signal = sensors.find_distress_signal(20);
    let distress_signal = sensors.find_distress_signal(4000000);

    distress_signal.x * 4000000 + distress_signal.y
}
