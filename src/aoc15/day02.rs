use crate::aoc::{io, conv};
use crate::aoc::grid::Dims3;

pub fn solve() {
    let items = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for dims in items {
        let (l, w, h) = dims;
        let (lw, wh, lh) = (l*w, w*h, l*h);
        let areas = [lw, wh, lh];
        let min_area = areas.iter().min().unwrap();
        total1 += (2 * lw) + (2 * wh) + (2 * lh) + min_area;

        let mut d = [l, w, h];
        d.sort();
        total2 += (2 * (d[0] + d[1])) + (l * w * h);
    }
    println!("{}", total1);
    println!("{}", total2);
}

fn data(full: bool) -> Vec<Dims3> {
    io::read_lines(full)
    .iter()
    .map(|x| conv::to_dims3(x, "x"))
    .collect()
}