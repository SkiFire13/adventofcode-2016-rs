#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a str>;

pub fn input_generator(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|line| {
            let (cn, ch) = line
                .as_bytes()
                .split(|&c| c == b'[' || c == b']')
                .map(|seq| {
                    seq.len() >= 4
                        && seq.windows(4).any(|chunk| {
                            let [a, b, c, d]: [_; 4] = chunk.try_into().unwrap();
                            a == d && b == c && a != b
                        })
                })
                .enumerate()
                .fold((0, 0), |(cn, ch), (i, abba)| {
                    if i % 2 == 0 {
                        (cn + abba as usize, ch)
                    } else {
                        (cn, ch + abba as usize)
                    }
                });
            cn >= 1 && ch == 0
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|line| {
            let abs = line
                .as_bytes()
                .split(|&c| c == b'[' || c == b']')
                .step_by(2)
                .flat_map(|seq| seq.windows(3))
                .filter_map(|chunk| {
                    let [a, b, a2]: [_; 3] = chunk.try_into().unwrap();
                    if a == a2 && a != b {
                        Some((a, b))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();

            line.as_bytes()
                .split(|&c| c == b'[' || c == b']')
                .skip(1)
                .step_by(2)
                .flat_map(|seq| seq.windows(3))
                .filter_map(|chunk| {
                    let [b, a, b2]: [_; 3] = chunk.try_into().unwrap();
                    if b == b2 && a != b {
                        Some((a, b))
                    } else {
                        None
                    }
                })
                .any(|(a, b)| abs.contains(&(a, b)))
        })
        .count()
}
