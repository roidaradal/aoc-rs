use crate::aoc::{Solution, io};


pub fn solve() -> Solution {
    let ranges = data(true);
    let mut total1: i64 = 0;
    let mut total2: i64 = 0;
    for (first, last) in ranges {
        for x in first..last+1 {
            if is_invalid(x) {
                total1 += x;
            }
            if is_invalid2(x) {
                total2 += x;
            }
        }
    }
    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for pair in io::first_line(25, 2, full).split(",") {
        let parts: Vec<&str> = pair.split("-").collect();
        let first: i64 = parts[0].parse().unwrap();
        let last: i64 = parts[1].parse().unwrap();
        ranges.push((first, last));
    }
    ranges
}

fn is_invalid(x: i64) -> bool {
    let s = x.to_string();
    let length = s.len();
    if length % 2 != 0 {
        return false
    }
    let half = length / 2;
    s[..half] == s[half..]
}

fn is_invalid2(x: i64) -> bool {
    let s = x.to_string();
    let length = s.len();
    let half = length / 2;
    for w in 1..half+1 {
        if length % w != 0 {
            continue
        }
        let r = length / w;
        if s[..w].repeat(r) == s {
            return true;
        }
    }
    false
}