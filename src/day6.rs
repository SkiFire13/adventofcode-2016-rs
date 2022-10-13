#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a str>;

pub fn input_generator(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> String {
    let mut counts = vec![HashMap::new(); input[0].len()];

    for line in input {
        for (c, count) in line.chars().zip(&mut counts) {
            *count.entry(c).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .map(|count| {
            count
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}

pub fn part2(input: &Input) -> String {
    let mut counts = vec![HashMap::new(); input[0].len()];

    for line in input {
        for (c, count) in line.chars().zip(&mut counts) {
            *count.entry(c).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .map(|count| {
            count
                .into_iter()
                .min_by_key(|&(_, count)| count)
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}
