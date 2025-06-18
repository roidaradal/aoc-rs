use itertools::Itertools;
use crate::aoc::{conv, io, Solution};

pub fn solve() -> Solution {
    let numbers_list = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for numbers in numbers_list {
        // Part 1
        let min = numbers.iter().min().unwrap();
        let max = numbers.iter().max().unwrap();
        total1 += max - min;

        // Part 2
        for mut pair in numbers.iter().combinations(2) {
            pair.sort();
            let (a, b) = (pair[0], pair[1]);
            if b % a == 0 {
                total2 += b / a;
                break;
            }
        }
    }
    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<Vec<u32>> {
    io::read_lines(17, 2, full)
        .iter()
        .map(|x| conv::to_vec_int(x, " "))
        .collect()
}
