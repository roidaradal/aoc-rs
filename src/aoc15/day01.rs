use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let line = data(true);
    let level1 = elevator_floor(line, None);

    // Part 2
    let line = data(true);
    let level2 = elevator_floor(line, Some(-1));

    io::solution(level1, level2)
}

fn data(full: bool) -> String {
    io::first_line(15, 1, full)
}

fn elevator_floor(line: String, goal: Option<i32>) -> i32 {
    let has_goal = goal.is_some();
    let goal = goal.unwrap_or(0);
    let mut level = 0;
    for (i, x) in line.chars().enumerate() {
        level += if x == '(' { 1 } else { - 1 };
        if has_goal && goal == level {
            return 1 + i as i32;
        }
    }
    level
}