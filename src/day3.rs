#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<[u32; 3]>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|n| n.trim().parse().expect("Invalid input"));
            [
                sides.next().expect("Invalid input"),
                sides.next().expect("Invalid input"),
                sides.next().expect("Invalid input"),
            ]
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|&&[a, b, c]| a < b + c && b < a + c && c < a + b)
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .chunks_exact(3)
        .flat_map(|chunk| array::from_fn::<_, 3, _>(|i| array::from_fn(|j| chunk[j][i])))
        .filter(|&[a, b, c]| a < b + c && b < a + c && c < a + b)
        .count()
}
