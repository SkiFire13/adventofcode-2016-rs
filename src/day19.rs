#[allow(unused_imports)]
use super::prelude::*;
type Input = u32;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Invalid input")
}

pub fn part1(input: &Input) -> u32 {
    let mut queue = (1..=*input).collect::<VecDeque<_>>();

    while queue.len() != 1 {
        queue.rotate_left(1);
        queue.pop_front();
    }

    queue[0]
}

pub fn part2(input: &Input) -> u32 {
    let mut queue = (1..=*input).collect::<VecDeque<_>>();

    queue.rotate_left(queue.len() / 2);

    while queue.len() != 2 {
        queue.pop_front();
        queue.rotate_left(1 - queue.len() % 2);
    }

    queue.pop_front();

    queue[0]
}
