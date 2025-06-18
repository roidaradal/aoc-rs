use crate::aoc::{conv, io, Solution};
use crate::aoc::grid::Dims3;

pub fn solve() -> Solution {
    let items = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for dims in items {
        // Part 1
        let (l, w, h) = dims;
        let (lw, wh, lh) = (l*w, w*h, l*h);
        let areas = [lw, wh, lh];
        let min_area = areas.iter().min().unwrap();
        total1 += (2 * lw) + (2 * wh) + (2 * lh) + min_area;

        // Part 2
        let mut d = [l, w, h];
        d.sort();
        total2 += (2 * (d[0] + d[1])) + (lw * h);
    }
    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<Dims3> {
    io::read_lines(15, 2, full)
        .iter()
        .map(|x| conv::to_dims3(x, "x"))
        .collect()
}