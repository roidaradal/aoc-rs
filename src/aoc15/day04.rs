use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    // Part 1
    let key = data(true);
    let idx1 = find_hash(key, 5);

    // Part 2
    let key = data(true);
    let idx2 = find_hash(key, 6);

    io::solution(idx1, idx2)
}

fn data(full: bool) -> String {
    io::first_line(15, 4, full)
}

fn find_hash(key: String, num_zeros: usize) -> u32 {
    let goal: String = strings::repeat_string("0", num_zeros);
    let mut i: u32 = 1;
    loop {
        let word = format!("{}{}", key, i);
        let hash = strings::md5_hash(&word);
        if hash.starts_with(&goal) {
            return i;
        }
        i += 1;
    }
}