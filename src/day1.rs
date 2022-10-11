#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(Direction, i32)>;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn apply_to(self, (dx, dy): (i32, i32)) -> (i32, i32) {
        match self {
            Self::Left => (-dy, dx),
            Self::Right => (dy, -dx),
        }
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .split(", ")
        .map(|mov| {
            let (dir, length) = mov.split_at(1);
            let dir = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid input"),
            };
            let length = length.parse().expect("Invalid input");
            (dir, length)
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut orientation = (0, 1);
    let mut pos = (0, 0);
    for &(dir, length) in input {
        orientation = dir.apply_to(orientation);
        pos.0 += length * orientation.0;
        pos.1 += length * orientation.1;
    }
    pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &Input) -> i32 {
    let mut orientation = (0, 1);
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    for &(dir, length) in input {
        orientation = dir.apply_to(orientation);
        for i in 0..length {
            let mid = (pos.0 + i * orientation.0, pos.1 + i * orientation.1);
            if !visited.insert(mid) {
                return mid.0.abs() + mid.1.abs();
            }
        }
        pos.0 += length * orientation.0;
        pos.1 += length * orientation.1;
    }
    panic!("No location visited twice")
}
