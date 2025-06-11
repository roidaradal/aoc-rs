use crate::aoc::io;
use crate::aoc::grid::{Dims3, to_dims3};

pub fn solve() {
    let items = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for dims in items.iter() {
        let (l, w, h) = dims;
        let (lw, wh, lh) = (l*w, w*h, l*h);
        let areas = [lw, wh, lh];
        let min_area = areas.iter().min().unwrap();
        total1 += (2 * lw) + (2 * wh) + (2 * lh) + min_area;

        let mut sides = [l, w, h];
        sides.sort();
        total2 += (2 * (sides[0] + sides[1])) + (l * w * h);
    }
    println!("{}", total1);
    println!("{}", total2);
}

fn data(full: bool) -> Vec<Dims3> {
    io::read_lines(full)
    .iter()
    .map(|x| to_dims3(x, "x"))
    .collect()
}