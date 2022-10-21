#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
    InvRotateLetter(char),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("swap position ") {
                let (p1, p2) = rest.split_once(" with position ").expect("Invalid input");
                let p1 = p1.parse().expect("Invalid input");
                let p2 = p2.parse().expect("Invalid input");
                Instruction::SwapPos(p1, p2)
            } else if let Some(rest) = line.strip_prefix("swap letter ") {
                let (l1, l2) = rest.split_once(" with letter ").expect("Invalid input");
                let l1 = l1.parse().expect("Invalid input");
                let l2 = l2.parse().expect("Invalid input");
                Instruction::SwapLetter(l1, l2)
            } else if let Some(rest) = line.strip_prefix("rotate left ") {
                let steps = rest.trim_end_matches('s').trim_end_matches(" step");
                let steps = steps.parse().expect("Invalid input");
                Instruction::RotateLeft(steps)
            } else if let Some(rest) = line.strip_prefix("rotate right ") {
                let steps = rest.trim_end_matches('s').trim_end_matches(" step");
                let steps = steps.parse().expect("Invalid input");
                Instruction::RotateRight(steps)
            } else if let Some(rest) = line.strip_prefix("rotate based on position of letter ") {
                let letter = rest.parse().expect("Invalid input");
                Instruction::RotateLetter(letter)
            } else if let Some(rest) = line.strip_prefix("reverse positions ") {
                let (p1, p2) = rest.split_once(" through ").expect("Invalid input");
                let p1 = p1.parse().expect("Invalid input");
                let p2 = p2.parse().expect("Invalid input");
                Instruction::Reverse(p1, p2)
            } else if let Some(rest) = line.strip_prefix("move position ") {
                let (p1, p2) = rest.split_once(" to position ").expect("Invalid input");
                let p1 = p1.parse().expect("Invalid input");
                let p2 = p2.parse().expect("Invalid input");
                Instruction::Move(p1, p2)
            } else {
                panic!("Invalid input");
            }
        })
        .collect()
}

const LEN: usize = 8;
fn execute(initial: &[u8; LEN], instrs: impl Iterator<Item = Instruction>) -> String {
    let mut idx_to_letter = initial.map(|c| (c - b'a') as usize);
    let mut letter_to_idx = [0; LEN];
    (0..LEN).for_each(|p| letter_to_idx[idx_to_letter[p]] = p);

    for instr in instrs {
        match instr {
            Instruction::SwapPos(p1, p2) => {
                idx_to_letter.swap(p1, p2);
                letter_to_idx[idx_to_letter[p1]] = p1;
                letter_to_idx[idx_to_letter[p2]] = p2;
            }
            Instruction::SwapLetter(l1, l2) => {
                let (l1, l2) = ((l1 as u8 - b'a') as usize, (l2 as u8 - b'a') as usize);
                letter_to_idx.swap(l1, l2);
                idx_to_letter[letter_to_idx[l1]] = l1;
                idx_to_letter[letter_to_idx[l2]] = l2;
            }
            Instruction::RotateLeft(n) => {
                idx_to_letter.rotate_left(n);
                letter_to_idx = letter_to_idx.map(|i| (i + LEN - n) % LEN);
            }
            Instruction::RotateRight(n) => {
                idx_to_letter.rotate_right(n);
                letter_to_idx = letter_to_idx.map(|i| (i + n) % LEN);
            }
            Instruction::RotateLetter(l) => {
                let l = (l as u8 - b'a') as usize;
                let p = letter_to_idx[l];
                let rot = (1 + p + (p >= 4) as usize) % LEN;
                idx_to_letter.rotate_right(rot);
                letter_to_idx = letter_to_idx.map(|i| (i + rot) % LEN);
            }
            Instruction::Reverse(p1, p2) => {
                idx_to_letter[p1..=p2].reverse();
                (p1..=p2).for_each(|p| letter_to_idx[idx_to_letter[p]] = p);
            }
            Instruction::Move(p1, p2) => {
                if p1 < p2 {
                    idx_to_letter[p1..=p2].rotate_left(1);
                    (p1..=p2).for_each(|p| letter_to_idx[idx_to_letter[p]] = p);
                } else {
                    idx_to_letter[p2..=p1].rotate_right(1);
                    (p2..=p1).for_each(|p| letter_to_idx[idx_to_letter[p]] = p);
                }
            }
            Instruction::InvRotateLetter(l) => {
                let l = (l as u8 - b'a') as usize;
                let p = letter_to_idx[l];
                let orig_p = (0..LEN)
                    .filter(|&i| (i + 1 + i + (i >= 4) as usize) % LEN == p)
                    .exactly_one()
                    .expect("Invalid input");
                let rot = (1 + orig_p + (orig_p >= 4) as usize) % LEN;
                idx_to_letter.rotate_left(rot);
                letter_to_idx = letter_to_idx.map(|i| (i + LEN - rot) % LEN);
            }
        }
    }

    idx_to_letter
        .into_iter()
        .map(|l| (l as u8 + b'a') as char)
        .collect()
}

pub fn part1(input: &Input) -> String {
    execute(b"abcdefgh", input.iter().copied())
}

pub fn part2(input: &Input) -> String {
    execute(
        b"fbgdceah",
        input.iter().rev().map(|&instr| match instr {
            Instruction::RotateLeft(n) => Instruction::RotateRight(n),
            Instruction::RotateRight(n) => Instruction::RotateLeft(n),
            Instruction::RotateLetter(l) => Instruction::InvRotateLetter(l),
            Instruction::Move(p1, p2) => Instruction::Move(p2, p1),
            instr => instr,
        }),
    )
}
