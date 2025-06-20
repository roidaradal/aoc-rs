use std::{cmp, collections::HashMap};
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    let seats = data(true);

    let mut ids: Vec<u32> = Vec::new();
    let mut max_id: u32 = 0;
    for seat in seats {
        let id = compute_id(seat);
        ids.push(id);
        // Part 1
        max_id = cmp::max(max_id, id);
    }

    // Part 2 
    ids.sort();
    let mut seat_id = 0;
    for p in ids.windows(2) {
        let (prev, curr) = (p[0], p[1]);
        if curr - prev > 1 {
            seat_id = curr-1;
            break;
        }
    }

    io::solution(max_id, seat_id)
}

fn data(full: bool) -> Vec<Seat> {
    let side: HashMap<char, u32> = HashMap::from([
        ('F', 0), ('B', 1), ('L', 0), ('R', 1),
    ]);
    io::read_lines(20, 5, full)
        .iter()
        .map(|line| {
            let row: Vec<u32> = line[..7].chars().map(|x| side[&x]).collect();
            let col: Vec<u32> = line[7..].chars().map(|x| side[&x]).collect();
            (row, col)
        })
        .collect()
}

type Seat = (Vec<u32>, Vec<u32>);

fn compute_id(seat: Seat) -> u32 {
    let (num_rows, num_cols) = (128, 8);
    let (rows, cols) = seat;
    let row = binary_moves(rows, num_rows);
    let col = binary_moves(cols, num_cols);
    (row * 8) + col
}

fn binary_moves(sides: Vec<u32>, limit: u32) -> u32 {
    let last = sides.len() - 1;
    let mut start= 0;
    let mut end = limit;
    for s in sides[..last].iter() {
        let mid = start + ((end-start) / 2);
        match s {
            0 => end = mid,
            1 => start = mid,
            _ => continue,
        }
    }
    if sides[last] == 0 { start } else { end - 1 }
}