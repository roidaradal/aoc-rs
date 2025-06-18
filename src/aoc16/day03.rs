use crate::aoc::{conv, io, Solution};
use crate::aoc::grid::Dims3;

pub fn solve() -> Solution {
    // Part 1
    let triples = data(true);
    let count1 = count_valid_triples(triples);
    
    // Part 2
    let triples = data(true);
    let triples = read_vertical(triples);
    let count2 = count_valid_triples(triples);

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<Dims3> {
    io::read_lines(16, 3, full)
        .iter()
        .map(|x| conv::to_dims3(x, " "))
        .collect()
}

fn count_valid_triples(triples: Vec<Dims3>) -> u32 {
    let mut count: u32 = 0;
    for triple in triples {
        let (a, b, c) = triple;
        if a+b > c && b+c > a && a+c > b {
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