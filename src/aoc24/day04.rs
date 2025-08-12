use crate::aoc::{grid, io, strings, Solution};
use crate::aoc::grid::{Coords, Delta, Dims2};

type Vector = (Coords, Delta);

pub fn solve() -> Solution {
    let board = data(true);
    let bounds = (board.len() as u32, board[0].len() as u32);
    let rows = bounds.0 as usize;

    // Part 1 
    let mut points: Vec<Coords> = Vec::new();
    for row in 0..rows {
        for (col, tile) in board[row].char_indices() {
            if tile == 'X' {
                points.push((row as i32, col as i32));
            }
        }
    }
    let mut vectors: Vec<Vector> = Vec::new();
    for center in points {
        for pt in grid::surround8(center) {
            if !grid::inside_bounds(pt, bounds) {
                continue;
            }
            let (row, col) = (pt.0 as usize, pt.1 as usize);
            if strings::nth_char(&board[row], col) == 'M' {
                vectors.push((pt, grid::get_delta(center, pt)));
            }
        }
    }
    for letter in "AS".chars() {
        vectors = find_next_positions(&board, bounds, vectors, letter)
    }
    let count1 = vectors.len();

    // Part 2 
    let min_bounds: Dims2 = (1, 1);
    let max_bounds: Dims2 = (bounds.0-1, bounds.1-1);
    let mut points: Vec<Coords> = Vec::new();
    for row in 0..rows {
        for (col, tile) in board[row].char_indices() {
            let pt = (row as i32, col as i32);
            if tile == 'A' && grid::inside_bounds_full(pt, min_bounds, max_bounds) {
                points.push(pt);
            }
        }
    }
    let mut count2 = 0;
    for pt in points {
        let (row, col) = (pt.0 as usize, pt.1 as usize);
        // Left diag
        let tl = strings::nth_char(&board[row-1], col-1);
        let br = strings::nth_char(&board[row+1], col+1);
        let ldiag: String = vec![tl, 'A', br].iter().collect();
        // Right diag 
        let tr = strings::nth_char(&board[row-1], col+1);
        let bl = strings::nth_char(&board[row+1], col-1);
        let rdiag: String = vec![tr, 'A', bl].iter().collect();
        if is_xmas(ldiag, rdiag) {
            count2 += 1
        }

    }

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(24, 4, full)
}

fn find_next_positions(board: &Vec<String>, bounds: Dims2, vectors: Vec<Vector>, letter: char) -> Vec<Vector> {
    let mut vectors2: Vec<Vector> = Vec::new();
    for (c, d) in vectors {
        let c = grid::step(c, d);
        if !grid::inside_bounds(c, bounds) {
            continue;
        }
        let (row, col) = (c.0 as usize, c.1 as usize);
        if strings::nth_char(&board[row], col) == letter {
            vectors2.push((c, d));
        }
    }
    vectors2
}

fn is_xmas(diag1: String, diag2: String) -> bool {
    let mas: String = String::from("MAS");
    let sam: String = String::from("SAM");
    (diag1 == mas || diag1 == sam) && (diag2 == mas || diag2 == sam)
}