use crate::aoc::{Solution, io, strings};

pub fn solve() -> Solution {
    let moves = data(true);

    let mut count1 = 0;
    let mut count2 = 0;
    let mut curr = 50;
    for (sign, repeat) in moves {
        for _ in 0..repeat {
            curr = (curr + sign) % 100;
            if curr == 0 {
                count2 += 1;
            }
        }
        if curr == 0 {
            count1 += 1;
        }
    }
    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<(i32, i32)> {
    let mut moves: Vec<(i32, i32)> = Vec::new();
    for line in io::read_lines(25, 1, full) {
        let sign = if strings::nth_char(&line, 0) == 'R' { 1 } else {-1};
        let steps: i32 = line[1..].parse().unwrap();
        moves.push((sign, steps));
    }
    moves
}