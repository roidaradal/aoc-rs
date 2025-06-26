use std::cmp;
use crate::aoc::{conv, grid, io, Solution};
use crate::aoc::grid::{Coords, Dims2, Grid};

pub fn solve() -> Solution {
    // Part 1
    let (lines, bounds) = data(true);
    let count1 = count_intersection(lines, bounds, false);

    // Part 2 
    let (lines, bounds) = data(true);
    let count2 = count_intersection(lines, bounds, true);

    io::solution(count1, count2)
}

type Line = (Coords, Coords);

fn data(full: bool) -> (Vec<Line>, Dims2) {
    let mut rows = 0;
    let mut cols = 0;
    let mut lines: Vec<Line> = Vec::new();
    for line in io::read_lines(21, 5, full) {
        let p: Vec<&str> = line.split("->").map(str::trim).collect();
        let (y1, x1) = conv::to_coords(&p[0], ",", false);
        let (y2, x2) = conv::to_coords(&p[1], ",", false);
        let (y, x) = (cmp::max(y1, y2), cmp::max(x1, x2));
        rows = cmp::max(rows, y);
        cols = cmp::max(cols, x);
        lines.push(((y1,x1), (y2,x2)));
    }
    rows += 1;
    cols += 1;
    (lines, (rows as u32, cols as u32))
}

fn count_intersection(lines: Vec<Line>, bounds: Dims2, with_diagonal: bool) -> u32 {
    let (rows, cols) = (bounds.0 as usize, bounds.1 as usize);
    let mut g: Grid<u32> = grid::new(0, rows, cols);
    for ((y1,x1), (y2,x2)) in lines {
        if x1 == x2 {
            add_vertical(&mut g, y1, y2, x1);
        } else if y1 == y2 {
            add_horizontal(&mut g, x1, x2, y1);
        } else if with_diagonal && y2.abs_diff(y1) == x2.abs_diff(x1) {
            add_diagonal(&mut g, x1, y1, x2, y2);
        }
    }
    g.into_iter().map(|line| {
        line.into_iter().map(|x| if x > 1 { 1 } else { 0 }).sum::<u32>()
    })
    .sum()
}

fn add_vertical(g: &mut Grid<u32>, y1: i32, y2: i32, x: i32) {
    let start = cmp::min(y1, y2);
    let end = cmp::max(y1, y2) + 1;
    let x = x as usize;
    for y in start..end {
        let y = y as usize;
        g[y][x] += 1;
    }
}

fn add_horizontal(g: &mut Grid<u32>, x1: i32, x2: i32, y: i32) {
    let start = cmp::min(x1, x2);
    let end = cmp::max(x1, x2) + 1;
    let y = y as usize;
    for x in start..end {
        let x = x as usize;
        g[y][x] += 1;
    }
}

fn add_diagonal(g: &mut Grid<u32>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let xs: Vec<i32> = if x1 < x2 { (x1..=x2).collect() } else { (x2..=x1).rev().collect() }; 
    let ys: Vec<i32> = if y1 < y2 { (y1..=y2).collect() } else { (y2..=y1).rev().collect() };
    let limit = xs.len();
    for i in 0..limit {
        let (y, x) = (ys[i] as usize, xs[i] as usize);
        g[y][x] += 1;
    }
}