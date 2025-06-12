use crate::aoc::{io, conv};
use crate::aoc::grid::Dims3;

pub fn solve() {
    let triples = data(true);
    let count = count_valid_triples(triples);
    println!("{}", count);
    
    let triples = data(true);
    let triples = read_vertical(triples);
    let count = count_valid_triples(triples);
    println!("{}", count);
}

fn data(full: bool) -> Vec<Dims3> {
    io::read_lines(full)
    .iter()
    .map(|x| conv::to_dims3(x, " "))
    .collect()
}

fn is_valid(t: Dims3) -> bool {
    let (a, b, c) = t;
    a+b > c && b+c > a && a+c > b
}

fn count_valid_triples(triples: Vec<Dims3>) -> u32 {
    let mut count: u32 = 0;
    for triple in triples {
        if is_valid(triple) {
            count += 1;
        }
    }
    count
}

fn read_vertical(triples: Vec<Dims3>) -> Vec<Dims3> {
    let mut triples2: Vec<Dims3> = Vec::new();
    for r in (0..triples.len()).step_by(3) {
        let (c00, c01, c02) = triples[r];
        let (c10, c11, c12) = triples[r+1];
        let (c20, c21, c22) = triples[r+2];
        triples2.push((c00, c10, c20));
        triples2.push((c01, c11, c21));
        triples2.push((c02, c12, c22));
    }
    triples2
}