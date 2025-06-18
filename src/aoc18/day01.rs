use std::collections::HashSet;
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let numbers = data(true);
    let total: i32 = numbers.iter().sum();

    // Part 2
    let limit = numbers.len();
    let mut i: usize = 0;
    let mut curr = 0;
    let mut done: HashSet<i32> = HashSet::new();
    loop {
        curr += numbers[i];
        if done.contains(&curr) {
            break;
        }
        done.insert(curr);
        i = (i+1) % limit;
    }

    io::solution(total, curr)
}

fn data(full: bool) -> Vec<i32> {
    io::read_lines(18, 1, full)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}