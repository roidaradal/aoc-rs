use crate::aoc::{grid, io, strings, Solution};
use crate::aoc::grid::{Coords, Delta};

pub fn solve() -> Solution {
    // Part 1 
    let count = count_slope((1,3), true);

    // Part 2 
    let mut product: u64 = 1;
    let deltas: Vec<Delta> = vec![(1,1), (1,3), (1,5), (1,7), (2,1)];
    for d in deltas {
        product *= count_slope(d, true) as u64;
    }

    io::solution(count, product)
}

fn data(full: bool, d: Delta) -> Vec<Vec<char>> {
    let lines = io::read_lines(20, 3, full);
    let (dy, dx) = d;
    let h = lines.len() as i32;
    let w = lines[0].len() as f64;
    let need_w = ((1 + dx) * num_steps(h, dy)) as f64;
    let repeat = (need_w / w).ceil() as usize;
    lines 
        .iter()
        .map(|line| strings::repeat_string(line, repeat).chars().collect())
        .collect()
}

fn num_steps(h: i32, dy: i32) -> i32 {
    (h-1) / dy
}

fn count_slope(d: Delta, full: bool) -> u32 {
    let g = data(full, d);
    let mut count: u32 = 0;
    let mut curr: Coords = (0, 0);
    let (h, dy) = (g.len() as i32, d.0);
    let steps = num_steps(h, dy);
    for _ in 0..steps {
        curr = grid::step(curr, d);
        let (row, col) = grid::coords_to_index(curr);
        if g[row][col] == '#' {
            count += 1;
        }
    }
    count
}