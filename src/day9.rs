#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a str;

pub fn input_generator(input: &str) -> Input<'_> {
    input
}

pub fn part1(input: &Input) -> usize {
    let mut count = 0;
    let mut input = *input;
    while !input.is_empty() {
        let (first, char_count, repeat_count, rest) = input
            .splitn(4, &['(', 'x', ')'][..])
            .collect_tuple()
            .expect("Invalid input");

        let char_count = char_count.parse().expect("Invalid input");
        let repeat_count: usize = repeat_count.parse().expect("Invalid input");

        input = &rest[char_count..];
        count += first.len() + char_count * repeat_count
    }

    count
}

pub fn part2(input: &Input) -> usize {
    fn expand_count(input: &str) -> usize {
        if !input.contains(&['(', 'x', ')'][..]) {
            return input.len();
        }

        let mut count = 0;
        let mut input = input;
        while !input.is_empty() {
            let (first, char_count, repeat_count, rest) = input
                .splitn(4, &['(', 'x', ')'][..])
                .collect_tuple()
                .expect("Invalid input");

            let char_count = char_count.parse().expect("Invalid input");
            let repeat_count: usize = repeat_count.parse().expect("Invalid input");
            let (repeated, rest) = rest.split_at(char_count);

            input = rest;
            count += first.len() + expand_count(repeated) * repeat_count
        }

        count
    }

    expand_count(input)
}
