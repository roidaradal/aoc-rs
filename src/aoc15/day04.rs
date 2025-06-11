use crate::aoc::{io, strings};

pub fn solve() {
    let key = data(true);
    let idx = find_hash(key, 5);
    println!("{}", idx);

    let key = data(true);
    let idx = find_hash(key, 6);
    println!("{}", idx);
}

fn data(full: bool) -> String {
    io::first_line(full)
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