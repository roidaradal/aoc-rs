use crate::aoc::io;

pub fn solve() {
    let word = data(true);
    let word = word.as_bytes();
    let limit = word.len();
    let mut total: u32 = 0;
    for i in 0..limit {
        let j = (i + 1) % limit;
        if word[i] == word[j] {
            total += parse_digit(word[i]);
        }
    }
    println!("{}", total);

    let mid = limit / 2;
    let mut total: u32 = 0;
    for i in 0..mid {
        let j = mid + i;
        if word[i] == word[j] {
            total += parse_digit(word[i]) * 2;
        }
    }
    println!("{}", total);
}

fn data(full: bool) -> String {
    io::first_line(full)
}

fn parse_digit(x: u8) -> u32 {
    (x as char).to_digit(10).unwrap()
}