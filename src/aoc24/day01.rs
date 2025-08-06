use std::collections::HashMap;
use crate::aoc::{conv, io, Solution};

pub fn solve() -> Solution {
    let (mut col1, mut col2) = data(true);

    // Part 1 
    col1.sort();
    col2.sort();
    let mut total1 = 0;
    for i in 0..col1.len() {
        total1 += col1[i].abs_diff(col2[i])
    }

    // Part 2 
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for x in col2 {
        *freq.entry(x).or_insert(0) +=1 ;
    }
    let mut total2 = 0;
    for x in col1 {
        total2 += x * freq.get(&x).unwrap_or(&0);
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> (Vec<i32>, Vec<i32>) {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for line in io::read_lines(24, 1, full) {
        let (a,b) = conv::to_int2(&line, " ");
        col1.push(a);
        col2.push(b);
    }
    (col1, col2)
} 