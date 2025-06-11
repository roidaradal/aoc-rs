use crate::aoc::io;

pub fn solve() {
    let line = data(true);
    let level = elevator_floor(&line, None);
    println!("{}", level);

    let level = elevator_floor(&line, Some(-1));
    println!("{}", level);
}

fn data(full: bool) -> String {
    io::first_line(full)
}

fn elevator_floor(line: &String, goal: Option<i32>) -> i32 {
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