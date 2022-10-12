#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<Room<'a>>;

pub struct Room<'a> {
    name: &'a str,
    sector_id: u32,
    checksum: [u8; 5],
}

pub fn input_generator(input: &str) -> Input<'_> {
    input
        .lines()
        .map(|line| {
            let line = line.trim_end_matches(']');
            let (rest, checksum) = line.rsplit_once('[').expect("Invalid input");
            let (name, sector_id) = rest.rsplit_once('-').expect("Invalid input");
            Room {
                name,
                sector_id: sector_id.parse().expect("Invalid input"),
                checksum: checksum.as_bytes().try_into().expect("Invalid input"),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|room| {
            let mut counts = [0; 26];
            for letter in room.name.bytes().filter(|&b| b != b'-') {
                counts[(letter - b'a') as usize] += 1;
            }

            let mut letter_and_count =
                counts.map_idx(|idx, count| (Reverse(count), b'a' + idx as u8));
            letter_and_count.sort_unstable();
            let expected_checksum = array::from_fn(|i| letter_and_count[i].1);

            (expected_checksum == room.checksum).then_some(room.sector_id)
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .find_map(|room| {
            room.name
                .bytes()
                .map(|b| {
                    if b == b'-' {
                        b' '
                    } else {
                        (((b - b'a') as u32 + room.sector_id) % 26) as u8 + b'a'
                    }
                })
                .zip("northpole object storage".bytes())
                .all(|(l, r)| l == r)
                .then_some(room.sector_id)
        })
        .expect("Invalid input")
}
