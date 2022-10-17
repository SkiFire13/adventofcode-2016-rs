#[allow(unused_imports)]
use super::prelude::*;
type Input = usize;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Invalid input")
}

fn is_open(x: usize, y: usize, magic_num: usize) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + magic_num).count_ones() % 2 == 0
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        Some((x + 1, y)),
        Some((x, y + 1)),
        (x != 0).then(|| (x - 1, y)),
        (y != 0).then(|| (x, y - 1)),
    ]
    .into_iter()
    .flatten()
}

pub fn part1(input: &Input) -> usize {
    let &magic_num = input;

    let mk_queue_elem = |x: usize, y: usize, steps| {
        let min_steps = steps + x.abs_diff(31) + y.abs_diff(39);
        (Reverse((min_steps, steps)), x, y)
    };
    let mut queue = BinaryHeap::from([mk_queue_elem(1, 1, 0)]);
    let mut seen = HashSet::new();

    while let Some((Reverse((_, steps)), x, y)) = queue.pop() {
        if seen.insert((x, y)) {
            if x == 31 && y == 39 {
                return steps;
            }

            for (x, y) in neighbors(x, y) {
                if is_open(x, y, magic_num) {
                    queue.push(mk_queue_elem(x, y, steps + 1));
                }
            }
        }
    }

    panic!("Invalid input");
}

pub fn part2(input: &Input) -> usize {
    let &magic_num = input;

    let mut queue = VecDeque::from([(0, 1, 1)]);
    let mut seen = HashSet::new();

    while let Some((steps @ 0..=50, x, y)) = queue.pop_front() {
        if seen.insert((x, y)) {
            for (x, y) in neighbors(x, y) {
                if is_open(x, y, magic_num) {
                    queue.push_back((steps + 1, x, y));
                }
            }
        }
    }

    seen.len()
}
