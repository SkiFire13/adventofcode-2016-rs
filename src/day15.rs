#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once("has ").expect("Invalid input");
            let (len, initial) = line
                .trim_end_matches(".")
                .split_once(" positions; at time=0, it is at position ")
                .expect("Invalid input");
            let len = len.parse().expect("Invalid input");
            let initial = initial.parse().expect("Invalid input");
            (len, initial)
        })
        .collect()
}

fn solve(congruences: impl Iterator<Item = (usize, usize)>) -> usize {
    congruences
        .enumerate()
        .map(|(i, (len, off))| (len, (len - (off + i + 1) % len) % len))
        .reduce(|(len1, off1), (len2, off2)| {
            let mut off = off1;
            while off % len2 != off2 {
                off += len1;
            }
            (len1 * len2, off)
        })
        .map(|(_, off)| off as usize)
        .expect("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    solve(input.iter().copied())
}

pub fn part2(input: &Input) -> usize {
    solve(input.iter().copied().chain([(11, 0)]))
}
