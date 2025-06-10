use crate::aoc::io;
use crate::aoc::grid::{Dims3, to_dims3};

pub fn solve() {
    let items = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for dims in items.iter() {
        let (l, w, h) = (dims.0, dims.1, dims.2);
        let (lw, wh, lh) = (l*w, w*h, l*h);
        let areas = vec![lw, wh, lh];
        let min_area = areas.iter().min().unwrap();
        total1 += (2 * lw) + (2 * wh) + (2 * lh) + min_area;

        let mut sides = vec![l, w, h];
        sides.sort();
        let d0 = sides.get(0).unwrap();
        let d1 = sides.get(1).unwrap();
        total2 += (2 * (d0 + d1)) + (l * w * h);
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