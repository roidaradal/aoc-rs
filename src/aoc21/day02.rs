use crate::aoc::{grid, io, Solution};
use crate::aoc::grid::{Delta, Coords};

pub fn solve() -> Solution {
    // Part 1
    let moves = data(true);
    let mut curr: Coords = (0,0);
    for d in moves {
        curr = grid::step(curr, d);
    }
    let (y, x) = curr;
    let part1 = y * x;

    // Part 2 
    let moves = data(true);
    let (mut y, mut x, mut a) = (0, 0, 0);
    for (dy, dx) in moves {
        if dy == 0 {
            x += dx;
            y += a * dx;
        } else {
            a += dy;
        }
    }
    let part2 = y * x;

    io::solution(part1, part2)
}

fn data(full: bool) -> Vec<Delta> {
    io::read_lines(21, 2, full)
        .iter()
        .map(|line| {
            let p: Vec<&str> = line.split_whitespace().collect();
            let x: i32 = p[1].parse().unwrap();
            match p[0] {
                "forward"   => (0, x),
                "up"        => (-x, 0),
                "down"      => (x, 0),
                _           => grid::X,
            }
        }).collect()
}