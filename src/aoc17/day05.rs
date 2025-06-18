use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let count1 = count_jumps(true, false);

    // Part 2
    let count2 = count_jumps(true, true);

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<i32> {
    io::read_lines(17, 5, full)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count_jumps(full: bool, clip: bool) -> u32 {
    let mut jumps = data(full);
    let limit = jumps.len() as i32;
    let mut i = 0;
    let mut count: u32 = 0;
    while 0 <= i && i < limit {
        let jump = jumps[i as usize];
        let increment = if clip && jump >= 3 { -1 } else { 1 };
        jumps[i as usize] = jump + increment;
        i += jump;
        count += 1;
    }
    count
}