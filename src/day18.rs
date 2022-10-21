#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Tile>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Safe,
    Trap,
}

pub fn input_generator(input: &str) -> Input {
    input
        .chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn solve(input: &[Tile], n: usize) -> usize {
    use Tile::*;

    let mut count = 0;
    let mut row = input.to_vec();
    let mut tmp = vec![Safe; input.len()];

    for _ in 0..n {
        count += row.iter().filter(|&&t| t == Safe).count();

        for i in 0..row.len() {
            let left = if i > 0 { row[i - 1] } else { Safe };
            let right = row.get(i + 1).copied().unwrap_or(Safe);

            tmp[i] = if left != right { Trap } else { Safe };
        }

        swap(&mut row, &mut tmp);
    }

    count
}

pub fn part1(input: &Input) -> usize {
    solve(input, 40)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 400_000)
}
