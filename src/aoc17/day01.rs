use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let word = data(true);
    let word = word.as_bytes();
    let limit = word.len();
    let mut total1: u32 = 0;
    for i in 0..limit {
        let j = (i + 1) % limit;
        if word[i] == word[j] {
            total1 += parse_digit(word[i]);
        }
    }

    // Part 2
    let mid = limit / 2;
    let mut total2: u32 = 0;
    for i in 0..mid {
        let j = mid + i;
        if word[i] == word[j] {
            total2 += parse_digit(word[i]) * 2;
        }
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> String {
    io::first_line(17, 1, full)
}

fn parse_digit(x: u8) -> u32 {
    (x as char).to_digit(10).unwrap()
}