#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Clone, Copy)]
pub enum Instruction {
    Rect { x: usize, y: usize },
    RotateRow { y: usize, count: usize },
    RotateColumn { x: usize, count: usize },
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match line {
            line if line.starts_with("rect ") => {
                let (x, y) = line[5..].split_once("x").expect("Invalid input");
                Instruction::Rect {
                    x: x.parse().expect("Invalid input"),
                    y: y.parse().expect("Invalid input"),
                }
            }
            line if line.starts_with("rotate row") => {
                let (y, count) = line[13..].split_once(" by ").expect("Invalid input");
                Instruction::RotateRow {
                    y: y.parse().expect("Invalid input"),
                    count: count.parse().expect("Invalid input"),
                }
            }
            line if line.starts_with("rotate column") => {
                let (x, count) = line[16..].split_once(" by ").expect("Invalid input");
                Instruction::RotateColumn {
                    x: x.parse().expect("Invalid input"),
                    count: count.parse().expect("Invalid input"),
                }
            }
            _ => panic!("Invalid input"),
        })
        .collect()
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;
fn execute_instructions(input: &Input) -> [bool; WIDTH * HEIGHT] {
    let mut grid = [false; WIDTH * HEIGHT];

    for &instruction in input {
        match instruction {
            Instruction::Rect { x, y } => {
                for j in 0..y {
                    for i in 0..x {
                        grid[i + j * WIDTH] = true;
                    }
                }
            }
            Instruction::RotateRow { y, count } => {
                grid[y * WIDTH..][..WIDTH].rotate_right(count);
            }
            Instruction::RotateColumn { x, count } => {
                let mut new_col = [false; HEIGHT];
                for i in 0..HEIGHT {
                    new_col[(i + count) % HEIGHT] = grid[x + i * WIDTH];
                }
                for i in 0..HEIGHT {
                    grid[x + i * WIDTH] = new_col[i];
                }
            }
        }
    }

    grid
}

pub fn part1(input: &Input) -> usize {
    execute_instructions(input)
        .into_iter()
        .filter(|&x| x)
        .count()
}

pub fn part2(input: &Input) -> String {
    ocr_bools_5x6(&execute_instructions(input))
}
