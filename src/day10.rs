#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<(u8, u8)>, HashMap<u8, (Receiver, Receiver)>);

#[derive(Clone, Copy)]
pub enum Receiver {
    Bot(u8),
    Output(u8),
}

pub fn input_generator(input: &str) -> Input {
    let mut initial = Vec::new();
    let mut rules = HashMap::new();

    for line in input.lines() {
        if line.starts_with("value") {
            let (value, bot) = line[6..]
                .split_once(" goes to bot ")
                .expect("Invalid input");
            let value = value.parse().expect("Invalid input");
            let bot = bot.parse().expect("Invalid input");
            initial.push((bot, value))
        } else if line.starts_with("bot") {
            let (bot, rest) = line[4..]
                .split_once(" gives low to ")
                .expect("Invalid input");
            let (low, high) = rest.split_once(" and high to ").expect("Invalid input");
            let parse_receiver = |rec: &str| {
                if rec.starts_with("bot") {
                    Receiver::Bot(rec[4..].parse().expect("Invalid input"))
                } else if rec.starts_with("output") {
                    Receiver::Output(rec[7..].parse().expect("Invalid input"))
                } else {
                    panic!("Invalid input")
                }
            };
            let bot = bot.parse().expect("Invalid input");
            let low = parse_receiver(low);
            let high = parse_receiver(high);
            assert!(!rules.contains_key(&bot));
            rules.insert(bot, (low, high));
        } else {
            panic!("Invalid input");
        }
    }

    (initial, rules)
}

pub fn part1(input: &Input) -> u8 {
    let (initial, rules) = input;
    let mut assignments = initial.clone();
    let mut states = Vec::new();

    while let Some((bot, value)) = assignments.pop() {
        if states.len() <= bot as usize {
            states.resize(bot as usize + 1, None);
        }

        match states[bot as usize] {
            Some(old) => {
                let low = min(value, old);
                let high = max(value, old);

                if low == 17 && high == 61 {
                    return bot;
                }

                states[bot as usize] = None;
                let (low_rec, high_rec) = rules[&bot];
                if let Receiver::Bot(rec) = low_rec {
                    assignments.push((rec, low));
                }
                if let Receiver::Bot(rec) = high_rec {
                    assignments.push((rec, high));
                }
            }
            None => states[bot as usize] = Some(value),
        }
    }

    panic!("Invalid input");
}

pub fn part2(input: &Input) -> u32 {
    let (initial, rules) = input;
    let mut assignments = initial.clone();
    let mut states = Vec::new();
    let mut out = 1;

    while let Some((bot, value)) = assignments.pop() {
        if states.len() <= bot as usize {
            states.resize(bot as usize + 1, None);
        }

        match states[bot as usize] {
            Some(old) => {
                let low = min(value, old);
                let high = max(value, old);

                states[bot as usize] = None;
                let (low_rec, high_rec) = rules[&bot];
                let mut handle_rec = |rec, val| {
                    match rec {
                        Receiver::Bot(rec) => assignments.push((rec, val)),
                        Receiver::Output(0 | 1 | 2) => out *= val as u32,
                        _ => {}
                    };
                };
                handle_rec(low_rec, low);
                handle_rec(high_rec, high);
            }
            None => states[bot as usize] = Some(value),
        }
    }

    out
}
