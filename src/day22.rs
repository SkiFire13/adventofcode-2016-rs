#[allow(unused_imports)]
use super::prelude::*;
type Input = HashMap<(u8, u8), (u16, u16)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let (path, _, used, avail, _) = line
                .split_whitespace()
                .collect_tuple()
                .expect("Invalid input");
            let (x, y) = path
                .trim_start_matches("/dev/grid/node-x")
                .split_once("-y")
                .expect("Invalid input");
            let x = x.parse().expect("Invalid input");
            let y = y.parse().expect("Invalid input");
            let used = used.trim_end_matches('T').parse().expect("Invalid input");
            let avail = avail.trim_end_matches('T').parse().expect("Invalid input");
            ((x, y), (used, avail))
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let useds = input
        .values()
        .map(|&(used, _)| used)
        .sorted_unstable()
        .collect::<Vec<_>>();
    let avails = input
        .values()
        .map(|&(_, avail)| avail)
        .sorted_unstable()
        .collect::<Vec<_>>();

    let double_counted = input
        .values()
        .filter(|&&(used, avail)| used != 0 && avail >= used)
        .count();

    let total = useds
        .into_iter()
        .skip_while(|&used| used == 0)
        .scan(&avails[..], |avails, used| {
            let idx = avails
                .iter()
                .position(|&avail| avail >= used)
                .unwrap_or(avails.len());
            *avails = &avails[idx..];
            Some(avails.len())
        })
        .sum::<usize>();

    total - double_counted
}

pub fn part2(input: &Input) -> usize {
    let maxx = input.keys().map(|&(x, _)| x).max().expect("Invalid input");
    let maxy = input.keys().map(|&(_, y)| y).max().expect("Invalid input");

    let (&empty, &(_, empty_size)) = input
        .iter()
        .filter(|&(_, &(used, _))| used == 0)
        .exactly_one()
        .expect("Invalid input");

    let locked = input
        .iter()
        .filter(|&(_, &(used, _))| used > empty_size)
        .map(|(&path, _)| path)
        .collect::<HashSet<_>>();

    let mk_elem = |steps, empty: (u8, u8), target: (u8, u8)| {
        let min_steps = steps                      // Existing steps
            + (target.0 as usize + target.1 as usize)     // Swaps
            + 4 * target.0.saturating_sub(1) as usize     // Min recover from x swap
            + 4 * target.1.saturating_sub(1) as usize     // Min recover from y swap
            + 2 * (target.0 > 0 && target.1 > 0) as usize // Min recover for turn
            + empty.0.abs_diff(target.0) as usize         // Empty reach target x
            + empty.1.abs_diff(target.1) as usize         // Empty reach target y
            + 1                                           // Empty reach next cell for swap
            - 2 * (empty.0 <= target.0 && empty.1 <= target.1) as usize; // Empty already ahead of target
        (Reverse((min_steps, steps)), empty, target)
    };

    let mut queue = BinaryHeap::from([mk_elem(0, empty, (maxx, 0))]);
    let mut seen = HashSet::new();

    while let Some((Reverse((_, steps)), empty, target)) = queue.pop() {
        if seen.insert((empty, target)) {
            if target == (0, 0) {
                return steps;
            }

            let (empty_x, empty_y) = empty;
            let (target_x, target_y) = target;

            if empty_x.abs_diff(target_x) + empty_y.abs_diff(target_y) == 1 {
                queue.push(mk_elem(steps + 1, target, empty));
            }

            [
                empty_x.checked_sub(1).map(|x| (x, empty_y)),
                empty_y.checked_sub(1).map(|y| (empty_x, y)),
                (empty_x + 1 < maxx).then_some((empty_x + 1, empty_y)),
                (empty_y + 1 < maxy).then_some((empty_x, empty_y + 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|new_empty| !locked.contains(new_empty))
            .filter(|&new_empty| new_empty != target)
            .for_each(|new_empty| queue.push(mk_elem(steps + 1, new_empty, target)));
        }
    }

    panic!("Invalid input")
}
