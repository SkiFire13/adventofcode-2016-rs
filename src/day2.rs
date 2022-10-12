#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<Direction>>;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut acc = 0;
    let mut pos = (1, 1);
    for instructions in input {
        for dir in instructions {
            let (dx, dy) = dir.delta();
            pos.0 = i32::clamp(pos.0 + dx, 0, 2);
            pos.1 = i32::clamp(pos.1 + dy, 0, 2);
        }
        acc = acc * 10 + pos.1 * 3 + pos.0 + 1;
    }
    acc
}

pub fn part2(input: &Input) -> String {
    let mut acc = String::with_capacity(input.len());
    let mut pos = (-2, 0);
    for instructions in input {
        for dir in instructions {
            let (dx, dy) = dir.delta();
            let new = (pos.0 + dx, pos.1 + dy);
            if new.0.abs() + new.1.abs() <= 2 {
                pos = new;
            }
        }
        const CHAR_MAPPING: [[char; 5]; 5] = [
            ['x', 'x', '1', 'x', 'x'],
            ['x', '2', '3', '4', 'x'],
            ['5', '6', '7', '8', '9'],
            ['x', 'A', 'B', 'C', 'x'],
            ['x', 'x', 'D', 'x', 'x'],
        ];
        acc.push(CHAR_MAPPING[(pos.1 + 2) as usize][(pos.0 + 2) as usize]);
    }
    acc
}
