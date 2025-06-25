use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    let numbers = data(true);
    let limit = numbers.len();
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 0..limit {
        // Part 1 
        if i < limit-1 && numbers[i+1] > numbers[i] {
            count1 += 1;
        }

        // Part 2 
        if i < 3 { continue; }
        let curr: u32 = numbers[i-2..i+1].iter().sum();
        let prev: u32 = numbers[i-3..i].iter().sum();
        if curr > prev {
            count2 += 1;
        }
    }

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<u32> {
    io::read_lines(21, 1, full)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}