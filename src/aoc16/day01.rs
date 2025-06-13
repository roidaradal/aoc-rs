use std::collections::HashSet;
use crate::aoc::{io, grid, strings, Int2};
use crate::aoc::grid::{Coords, Delta};

pub fn solve() {
    let moves = data(true);
    let hq = find_hq(moves, false);
    println!("{}", grid::manhattan_origin(hq));

    let moves = data(true);
    let hq = find_hq(moves, true);
    println!("{}", grid::manhattan_origin(hq));
}

const L: i32 = -1;
const R: i32 = 1;

fn data(full: bool) -> Vec<Int2> {
    io::first_line(full)
    .split(",")
    .map(|x| {
        let x = x.trim();
        let turn = if strings::nth_char(x, 0) == 'L' { L } else { R };
        let steps= x[1..].parse().unwrap();
        (turn, steps)
    })
    .collect()
}

fn find_hq(moves: Vec<Int2>, at_visited_twice: bool) -> Coords {
    let mut curr: Coords = (0, 0);
    let mut d: Delta = grid::X;
    let mut visited: HashSet<Coords> = HashSet::new();
    for m in moves {
        let (turn, steps) =  m;
        if d == grid::X {
            d = if turn == L { grid::L } else { grid::R };
        } else if turn == L {
            d = grid::left_of(d);
        } else if turn == R {
            d = grid::right_of(d);
        }
        for _ in 0..steps {
            curr = grid::step(curr, d);
            if at_visited_twice && visited.contains(&curr) {
                return curr
            }
            visited.insert(curr);
        }
    }
    curr
}