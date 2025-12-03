use crate::aoc::{Solution, conv, io};


pub fn solve() -> Solution {
    let banks = data(true);
    let total1: u64 = banks.iter().map(|bank| compute_total_jolt(bank, 2)).sum();
    let total2: u64 = banks.iter().map(|bank| compute_total_jolt(bank, 12)).sum();
    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<Vec<u32>> {
    io::read_lines(25, 3, full)
        .iter()
        .map(|line| conv::to_int_line(line))
        .collect()
}

fn compute_total_jolt(bank: &Vec<u32>, num_bat: usize) -> u64 {
    let n = bank.len();
    let mut start: usize = 0;
    let mut batteries: Vec<u32> = Vec::new();
    for d in 0..num_bat {
        let limit = n-num_bat+d+1;
        let mut candidates: Vec<(u32, i32)> = Vec::new();
        for i in start..limit {
            candidates.push((bank[i], -(i as i32)))
        }
        let (battery, index) = candidates.into_iter().max().unwrap();
        batteries.push(battery);
        start = (-index as usize) + 1;
    }
    batteries.into_iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}