#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u32, u32)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Invalid input");
            let start = start.parse().expect("Invalid input");
            let end = end.parse().expect("Invalid input");
            (start, end)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut ranges = input.to_vec();
    ranges.sort_unstable();

    let mut min_valid = 0;
    for (start, end) in ranges {
        let (start, end) = (start as u64, end as u64);

        if start > min_valid {
            return min_valid;
        }

        min_valid = max(min_valid, end + 1);
    }

    min_valid
}

pub fn part2(input: &Input) -> u64 {
    let mut ranges = input.to_vec();
    ranges.sort_unstable();

    let mut allowed = 0;
    let mut min_valid = 0;
    for (start, end) in ranges {
        let (start, end) = (start as u64, end as u64);

        if start > min_valid {
            allowed += start - min_valid;
        }

        min_valid = max(min_valid, end + 1);
    }

    allowed + (u32::MAX as u64).checked_sub(min_valid).unwrap_or(0)
}
