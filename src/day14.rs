#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a [u8];

pub fn input_generator(input: &str) -> Input<'_> {
    input.as_bytes()
    // b"abc"
}

fn fmt_to_buf(mut i: usize, buf: &mut [u8; 20]) -> &[u8] {
    let mut idx = 20;
    while i != 0 {
        idx -= 1;
        buf[idx] = (i % 10) as u8 + b'0';
        i /= 10;
    }
    &buf[idx..]
}

fn hash_hex_digit(hash: &[u8], i: usize) -> u8 {
    (hash[i / 2] >> (4 - 4 * (i % 2))) & 0b1111
}

fn repeats_of(hash: md5::digest::Output<md5::Md5>) -> impl Iterator<Item = (u8, usize)> {
    (0..2 * hash.len())
        .map(move |i| hash_hex_digit(&hash, i))
        .map(|b| (b, 1))
        .coalesce(|(old, count), (new, _)| match () {
            _ if old == new => Ok((old, count + 1)),
            _ => Err(((old, count), (new, 1))),
        })
}

fn find_64_key(mut hash: impl FnMut(usize) -> md5::digest::Output<md5::Md5>) -> usize {
    let mut key_idxs = Vec::with_capacity(70);
    let mut queue = VecDeque::new();
    let mut curr_idx = 0;

    loop {
        if let Some(&key_idx) = key_idxs.get(63) {
            if key_idx + 1000 < curr_idx {
                return key_idx;
            }
        }

        loop {
            match queue.front() {
                Some(&(_, front_idx)) if front_idx + 1000 < curr_idx => drop(queue.pop_front()),
                _ => break,
            }
        }

        let mut triplet_found = false;
        for (digit, count) in repeats_of(hash(curr_idx)) {
            if count >= 5 {
                for &(key_digit, key_idx) in &queue {
                    if key_digit == digit && key_idx != curr_idx && !key_idxs.contains(&key_idx) {
                        key_idxs.push(key_idx);
                    }
                }
                key_idxs.sort_unstable();
            }
            if count >= 3 && !triplet_found {
                triplet_found = true;
                queue.push_back((digit, curr_idx));
            }
        }

        curr_idx += 1;
    }
}

pub fn part1(input: &Input) -> usize {
    use md5::{Digest, Md5};

    let base = Md5::new_with_prefix(input);
    find_64_key(|idx| {
        let mut hasher = base.clone();
        hasher.update(fmt_to_buf(idx, &mut [0; 20]));
        hasher.finalize()
    })
}

pub fn part2(input: &Input) -> usize {
    use md5::{Digest, Md5};

    let base = Md5::new_with_prefix(input);
    find_64_key(|idx| {
        let mut hasher = base.clone();
        hasher.update(fmt_to_buf(idx, &mut [0; 20]));
        let mut hash = hasher.finalize_reset();
        for _ in 0..2016 {
            let buf: [_; 32] =
                array::from_fn(|i| b"0123456789abcdef"[hash_hex_digit(&hash, i) as usize]);
            hasher.update(&buf);
            hasher.finalize_into_reset(&mut hash);
        }
        hash
    })
}
