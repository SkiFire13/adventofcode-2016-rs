#[allow(unused_imports)]
use super::prelude::*;
type Input = State<INITIAL_ELEM_COUNT>;

const INITIAL_ELEM_COUNT: usize = 5;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct State<const ELEM_COUNT: usize>(u32);

impl<const ELEM_COUNT: usize> State<ELEM_COUNT> {
    fn with_floor_generic(self, id: usize, floor: u32) -> Self {
        assert!(id <= 2 * ELEM_COUNT);
        Self(self.0 & !(0b11 << (id * 2)) | (floor << (id * 2)))
    }
    fn with_gen_floor(self, id: usize, floor: u32) -> Self {
        self.with_floor_generic(2 * id, floor)
    }
    fn with_chip_floor(self, id: usize, floor: u32) -> Self {
        self.with_floor_generic(2 * id + 1, floor)
    }
    fn with_position(self, floor: u32) -> Self {
        self.with_floor_generic(2 * ELEM_COUNT, floor)
    }
    fn floor_of_generic(self, id: usize) -> u32 {
        assert!(id <= 2 * ELEM_COUNT);
        (self.0 >> (id * 2)) & 0b11
    }
    fn floor_of_gen(self, id: usize) -> u32 {
        self.floor_of_generic(2 * id)
    }
    fn floor_of_chip(self, id: usize) -> u32 {
        self.floor_of_generic(2 * id + 1)
    }
    fn floor_of_position(self) -> u32 {
        self.floor_of_generic(2 * ELEM_COUNT)
    }
    fn is_valid(self) -> bool {
        'id: for id in 0..ELEM_COUNT {
            let id_chip_floor = self.floor_of_chip(id);
            if id_chip_floor == self.floor_of_gen(id) {
                continue 'id;
            }

            for other_id in 0..ELEM_COUNT {
                if id != other_id && id_chip_floor == self.floor_of_gen(other_id) {
                    return false;
                }
            }
        }

        true
    }
    fn is_end(self) -> bool {
        let mut end = State::default().with_position(0b11);
        for id in 0..ELEM_COUNT {
            end = end.with_chip_floor(id, 0b11).with_gen_floor(id, 0b11);
        }
        self == end
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut state = State::default();
    let mut counter = 0;
    let mut id_map = HashMap::new();
    let mut get_id = |elem| {
        *id_map.entry(elem).or_insert_with(|| {
            assert!(counter < INITIAL_ELEM_COUNT);
            let id = counter;
            counter += 1;
            id
        })
    };

    for (floor, line) in input.lines().enumerate() {
        let (_, line) = line.split_once("contains").expect("Invalid input");
        let line = line.trim_end_matches(".");
        if line == " nothing relevant" {
            continue;
        }
        let (first, last) = line
            .split_once(" and")
            .map(|(first, last)| (first, Some(last)))
            .unwrap_or((line, None));
        for elem in first.trim_end_matches(",").split(",").chain(last) {
            let elem = elem.trim_start_matches(" and").trim_start_matches(" a ");
            if let Some(elem) = elem.strip_suffix(" generator") {
                state = state.with_gen_floor(get_id(elem), floor as u32);
            } else if let Some(elem) = elem.strip_suffix("-compatible microchip") {
                state = state.with_chip_floor(get_id(elem), floor as u32);
            } else {
                panic!("Invalid input");
            }
        }
    }

    assert_eq!(counter, INITIAL_ELEM_COUNT);

    state
}

fn solve<const ELEM_COUNT: usize>(state: State<ELEM_COUNT>) -> usize {
    let mut queue = VecDeque::from([(0, state)]);
    let mut seen = HashSet::new();

    while let Some((depth, state)) = queue.pop_front() {
        if seen.insert(state) {
            if state.is_end() {
                return depth;
            }

            let curr_floor = state.floor_of_position();

            macro_rules! enqueue_state {
                (|$state:ident, $new_floor:ident| $update:expr) => {
                    let mut update = |$new_floor| {
                        let $state = state.with_position($new_floor);
                        let new_state = $update;
                        if new_state.is_valid() {
                            queue.push_back((depth + 1, new_state))
                        }
                    };
                    if curr_floor > 0 {
                        update(curr_floor - 1);
                    }
                    if curr_floor < 3 {
                        update(curr_floor + 1);
                    }
                };
            }

            for id in 0..ELEM_COUNT {
                let chip_here = state.floor_of_chip(id) == curr_floor;
                let gen_here = state.floor_of_gen(id) == curr_floor;

                if chip_here && gen_here {
                    enqueue_state!(|state, new_floor| state
                        .with_chip_floor(id, new_floor)
                        .with_gen_floor(id, new_floor));
                }

                if chip_here {
                    enqueue_state!(|state, new_floor| state.with_chip_floor(id, new_floor));
                }

                if gen_here {
                    enqueue_state!(|state, new_floor| state.with_gen_floor(id, new_floor));

                    for other_id in 0..ELEM_COUNT {
                        if other_id != id && state.floor_of_gen(other_id) == curr_floor {
                            enqueue_state!(|state, new_floor| state
                                .with_gen_floor(id, new_floor)
                                .with_gen_floor(other_id, new_floor));
                        }
                    }
                }
            }
        }
    }

    panic!("Invalid input");
}

pub fn part1(input: &Input) -> usize {
    solve(*input)
}

pub fn part2(input: &Input) -> usize {
    let mut state = State::<7>::default();
    for id in 0..INITIAL_ELEM_COUNT {
        state = state
            .with_chip_floor(id, input.floor_of_chip(id))
            .with_gen_floor(id, input.floor_of_gen(id));
    }
    solve(state)
}
