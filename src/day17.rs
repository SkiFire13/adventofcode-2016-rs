#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a [u8];

pub fn input_generator(input: &str) -> Input<'_> {
    input.as_bytes()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Path {
    Nil,
    Cons(Direction, Rc<Path>),
}

#[ord_by_key(|e| Reverse(e.min_steps))]
struct Elem {
    min_steps: usize,
    steps: usize,
    x: usize,
    y: usize,
    path: Rc<Path>,
    state: md5::Md5,
}

const TARGET_X: usize = 3;
const TARGET_Y: usize = 3;
impl Elem {
    fn new(steps: usize, x: usize, y: usize, path: Rc<Path>, state: md5::Md5) -> Self {
        Self {
            min_steps: steps + TARGET_X - x + TARGET_Y - y,
            steps,
            x,
            y,
            path,
            state,
        }
    }
}

fn simulate<T>(input: &[u8], mut f: impl FnMut(&Path, usize) -> ControlFlow<T, T>) -> T {
    use md5::{Digest, Md5};

    let mut output = None;
    let mut queue = BinaryHeap::from([Elem::new(
        0,
        0,
        0,
        Rc::new(Path::Nil),
        Md5::new_with_prefix(input),
    )]);

    while let Some(elem) = queue.pop() {
        if elem.x == TARGET_X && elem.y == TARGET_Y {
            match f(&elem.path, elem.steps) {
                ControlFlow::Break(t) => return t,
                ControlFlow::Continue(t) => {
                    output = Some(t);
                    continue;
                }
            }
        }

        let hash = elem.state.clone().finalize();

        macro_rules! branch {
            ($xy:ident $glt:tt $n:tt && $h:expr => $d:ident $c:literal ($dox:tt $dnx:tt, $doy:tt $dny:tt)) => {
                if elem.$xy $glt $n && $h > 10 {
                    let path = Rc::new(Path::Cons(Direction::$d, elem.path.clone()));
                    let state = elem.state.clone().chain_update(std::slice::from_ref(&($c as u8)));
                    queue.push(Elem::new(elem.steps + 1, elem.x $dox $dnx, elem.y $doy $dny, path, state));
                }
            }
        }
        branch!(y > 0 && hash[0] >> 4 => Up 'U' (+ 0, - 1));
        branch!(y < 3 && hash[0] & 0b1111 => Down 'D' (+ 0, + 1));
        branch!(x > 0 && hash[1] >> 4 => Left 'L' (- 1, + 0));
        branch!(x < 3 && hash[1] & 0b1111 => Right 'R' (+ 1, - 0));
    }

    output.expect("No solution found")
}

pub fn part1(input: &Input) -> String {
    simulate(input, |path, _| {
        let mut s = Vec::new();
        let mut path = path;
        while let Path::Cons(d, next) = path {
            s.push(match d {
                Direction::Up => 'U',
                Direction::Down => 'D',
                Direction::Left => 'L',
                Direction::Right => 'R',
            });
            path = next;
        }
        ControlFlow::Break(s.into_iter().rev().collect())
    })
}

pub fn part2(input: &Input) -> usize {
    simulate(input, |_, steps| ControlFlow::Continue(steps))
}
