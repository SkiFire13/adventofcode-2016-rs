#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

pub fn input_generator(input: &str) -> Input {
    input.to_owned().into_bytes()
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

fn find_md5<F: FnMut(u8, u8) -> bool>(input: &[u8], mut f: F) {
    use md5::{Digest, Md5};

    let base_hasher = {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher
    };

    let mut idx = 0;

    loop {
        const CHUNK_SIZE: usize = 1_000_000;

        let chunk_result = (idx..idx + CHUNK_SIZE)
            .into_par_iter()
            .with_max_len(50_000)
            .find_map_first(|i| {
                let mut hasher = base_hasher.clone();
                hasher.update(fmt_to_buf(i, &mut [0; 20]));
                match &*hasher.finalize() {
                    &[0, 0, b3, b4, ..] if b3 < 16 => Some((i, b3, b4)),
                    _ => None,
                }
            });

        match chunk_result {
            Some((curr_idx, b3, b4)) => {
                idx = curr_idx + 1;
                if f(b3, b4) {
                    return;
                }
            }
            None => {
                idx += CHUNK_SIZE;
                continue;
            }
        }
    }
}

pub fn part1(input: &Input) -> String {
    let mut out = String::with_capacity(8);

    find_md5(input, |b3, _| {
        use std::fmt::Write;
        let _ = write!(&mut out, "{:x}", b3);
        out.len() == 8
    });

    out
}

pub fn part2(input: &Input) -> String {
    let mut out_len = 0;
    let mut out_raw = [None; 8];

    find_md5(input, |b3, b4| {
        if b3 < 8 && out_raw[b3 as usize].is_none() {
            out_raw[b3 as usize] = Some(b4 >> 4);
            out_len += 1;
        }
        out_len == 8
    });

    let mut out = String::with_capacity(8);
    for raw in out_raw {
        use std::fmt::Write;
        let _ = write!(&mut out, "{:x}", raw.unwrap());
    }
    out
}
