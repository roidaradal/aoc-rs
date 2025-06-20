use itertools::Itertools;
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let numbers = data(true);
    let value1 = find_2020_combo(numbers, 2); 

    // Part 2
    let numbers = data(true);
    let value2 = find_2020_combo(numbers, 3);

    io::solution(value1, value2)
}

fn data(full: bool) -> Vec<u32> {
    io::read_lines(20, 1, full)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_2020_combo(numbers: Vec<u32>, take: usize) -> u32 {
    for combo in numbers.into_iter().combinations(take) {
        let total: u32 = combo.iter().sum();
        if total == 2020 {
            return combo.iter().product();
        }
    }
    0
}