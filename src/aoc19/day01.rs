use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let numbers = data(true);
    let total1: u32 = numbers.into_iter().map(fuel).sum();

    // Part 2
    let numbers = data(true);
    let total2: u32 = numbers.into_iter().map(total_fuel).sum();

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<u32> {
    io::read_lines(19, 1, full)
    .iter()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn fuel(x: u32) -> u32 {
    let f = (x as f64 / 3.0).floor() - 2.0;
    if f <= 0.0 {
        return 0;
    }
    f as u32
}

fn total_fuel(mut x: u32) -> u32 {
    let mut total: u32 = 0;
    while x > 0 {
        x = fuel(x);
        total += x;
    }
    total
}