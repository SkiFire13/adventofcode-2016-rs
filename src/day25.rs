#[allow(unused_imports)]
use super::prelude::*;
type Input = (u32, u32);

pub fn input_generator(input: &str) -> Input {
    let (la, lb) = input.lines().skip(1).next_tuple().expect("Invalid input");
    let (_, a, _) = la.splitn(3, ' ').collect_tuple().expect("Invalid input");
    let (_, b, _) = lb.splitn(3, ' ').collect_tuple().expect("Invalid input");
    let a = a.parse().expect("Invalid input");
    let b = b.parse().expect("Invalid input");
    (a, b)
}

pub fn part1(input: &Input) -> u32 {
    let &(a, b) = input;
    let target = a * b;
    let mut sol = 0b10;
    while sol < target {
        sol = (sol << 2) | 0b10;
    }
    sol - target
}
