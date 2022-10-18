#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<bool>;

pub fn input_generator(input: &str) -> Input {
    input.trim().bytes().map(|b| b == b'1').collect()
}

// TODO: Optimize this bruteforce
fn dragon_curve(mut seq: Vec<bool>) -> Vec<bool> {
    seq.reserve(seq.len() + 1);
    seq.push(false);
    for i in (0..seq.len() - 1).rev() {
        seq.push(!seq[i]);
    }
    seq
}

fn checksum(seq: Vec<bool>) -> String {
    seq.chunks_exact(1 << seq.len().trailing_zeros())
        .map(|chunk| chunk.iter().fold(true, |acc, &b| acc ^ b))
        .map(|b| if b { '1' } else { '0' })
        .collect()
}

fn solve(input: &[bool], target_len: usize) -> String {
    let mut seq = input.to_vec();

    while seq.len() < target_len {
        seq = dragon_curve(seq);
    }

    checksum(seq[..target_len].to_vec())
}

pub fn part1(input: &Input) -> String {
    solve(input, 272)
}

pub fn part2(input: &Input) -> String {
    solve(input, 35651584)
}
