use crate::aoc;

pub fn solve() {
    let line = data(true);
    let level = elevator_floor(&line, None);
    println!("{}", level);

    let level = elevator_floor(&line, Some(-1));
    println!("{}", level);
}

fn data(full: bool) -> String {
    let lines = aoc::io::read_lines(full);
    lines.first().unwrap().to_string()
}

fn elevator_floor(line: &String, goal: Option<i32>) -> i32 {
    let mut level = 0;
    for (i, x) in line.chars().enumerate() {
        level += if x == '(' { 1 } else { - 1 };
        if let Some(goal) = goal {
            if goal == level {
                return 1 + (i as i32);
            }
        }
    }
    level
}