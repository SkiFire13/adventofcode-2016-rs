#[allow(unused_imports)]
use super::prelude::*;
type Input = (Grid<bool>, Vec<(usize, usize)>);

pub fn input_generator(input: &str) -> Input {
    let mut targets = HashMap::new();
    let grid = Grid::from_input_chars(input, |c, x, y| match c {
        '.' => true,
        '#' => false,
        c => {
            let id = c.to_digit(10).expect("Invalid input") as usize;
            targets.insert(id, (x, y));
            true
        }
    });
    let len = targets.keys().copied().max().expect("Invalid input");
    assert!(len < 10);
    let targets = (0..=len).map(|id| targets[&id]).collect();
    (grid, targets)
}

fn solve(input: &Input, mut is_end: impl FnMut(u16, u8) -> bool) -> usize {
    let (grid, targets) = input;

    let inverse_targets = targets
        .iter()
        .enumerate()
        .map(|(id, pos)| (pos, id))
        .collect::<HashMap<_, _>>();

    let reachable = targets
        .iter()
        .enumerate()
        .map(|(id, &start)| {
            let mut reachable = Vec::new();
            let mut queue = VecDeque::from([(0, start)]);
            let mut visited = grid.clone();

            while let Some((steps, pos)) = queue.pop_front() {
                if !visited[pos] {
                    continue;
                }
                visited[pos] = false;

                if let Some(&target) = inverse_targets.get(&pos) {
                    if target != id {
                        reachable.push((target, steps));
                        continue;
                    }
                }

                let mut push = |x, y| {
                    if visited[(x, y)] {
                        queue.push_back((steps + 1, (x, y)));
                    }
                };
                push(pos.0 - 1, pos.1);
                push(pos.0 + 1, pos.1);
                push(pos.0, pos.1 - 1);
                push(pos.0, pos.1 + 1);
            }

            reachable
        })
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::from([(Reverse(0), 1, 0)]);
    let mut seen = HashSet::new();

    while let Some((Reverse(steps), visited, curr)) = queue.pop() {
        if !seen.insert((visited, curr)) {
            continue;
        }

        if is_end(visited, curr) {
            return steps;
        }

        for &(neighbor, add_steps) in &reachable[curr as usize] {
            queue.push((
                Reverse(steps + add_steps),
                visited | (1 << neighbor),
                neighbor as u8,
            ));
        }
    }

    panic!("Invalid input");
}

pub fn part1(input: &Input) -> usize {
    let all_visited = (1u16 << input.1.len()) - 1;
    solve(input, |visited, _| visited == all_visited)
}

pub fn part2(input: &Input) -> usize {
    let all_visited = (1u16 << input.1.len()) - 1;
    solve(input, |visited, curr| visited == all_visited && curr == 0)
}
