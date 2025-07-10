use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1 
    let calories = data(true);
    let max_calories = calories.into_iter().max().unwrap();

    // Part 2 
    let mut calories = data(true);
    calories.sort_by(|a,b| b.cmp(a)); // reverse
    let top3: u32 = calories[0..3].into_iter().sum();

    io::solution(max_calories, top3)
}

fn data(full: bool) -> Vec<u32> {
    let mut calories: Vec<u32> = Vec::new();
    let mut curr: u32 = 0;
    for line in io::read_lines(22, 1, full) {
        if line == "" {
            calories.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>().unwrap();
        }
    }
    calories.push(curr);
    calories
}