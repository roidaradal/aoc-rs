use std::iter;

use crate::aoc::{io, str};

pub fn solve() {
    let key = data(true);

    let idx = find_hash(&key, 5);
    println!("{}", idx);

    let idx = find_hash(&key, 6);
    println!("{}", idx);
}

fn data(full: bool) -> String {
    let lines = io::read_lines(full);
    lines.first().unwrap().to_string()
}

fn find_hash(key: &String, num_zeros: usize) -> u32 {
    let goal: String = iter::repeat("0").take(num_zeros).collect();
    let mut i: u32 = 1;
    loop {
        let word = format!("{}{}", key, i);
        let hash = str::md5_hash(&word);
        if hash.starts_with(&goal) {
            return i;
        }
        i += 1;
    }
}