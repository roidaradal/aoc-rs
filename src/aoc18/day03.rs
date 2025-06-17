use std::collections::HashSet;
use crate::aoc::{conv, grid, io, Solution};
use crate::aoc::grid::{Coords, Dims2, Grid};

type Claim = (u32, Coords, Dims2);

pub fn solve() -> Solution {
    // Part 1
    let claims = data(true);
    let mut g: Grid<u32> = grid::new(0, 1000, 1000);
    for (_, (row, col), (h,w)) in claims {
        let (row, col) = (row as u32, col as u32);
        for dy in 0..h {
            let r = (row + dy) as usize;
            for dx in 0..w {
                let c = (col + dx) as usize;
                g[r][c] += 1;
            }
        }
    }
    let count: usize = g
    .iter()
    .map(|line| {
        line.iter().filter(|x| **x > 1).count()
    })
    .sum();

    // Part 2
    let claims = data(true);
    let mut g: Grid<u32> = grid::new(0, 1000, 1000);
    let mut clean: HashSet<u32> = HashSet::new();
    for (id, (row,col), (h, w)) in claims {
        let (row, col)= (row as u32, col as u32);
        let mut ok = true;
        for dy in 0..h {
            let r = (row + dy) as usize;
            for dx in 0..w {
                let c = (col + dx) as usize;
                if g[r][c] == 0 {
                    g[r][c] = id;
                } else {
                    ok = false;
                    let owner = g[r][c];
                    if clean.contains(&owner) {
                        clean.remove(&owner);
                    }
                }
            }
        }
        if ok {
            clean.insert(id);
        }
    }
    let ids: Vec<u32> = clean.into_iter().take(1).collect();
    let id = ids[0];
    
    io::solution(count, id)
}

fn data(full: bool) -> Vec<Claim> {
    io::read_lines(18, 3, full)
    .iter()
    .map(|x| new_claim(x))
    .collect()
}

fn new_claim(line: &String) -> Claim {
    let parts: Vec<&str> = line
    .split("@")
    .map(str::trim)
    .collect();
    let claim_id: u32 = parts[0].strip_prefix("#").unwrap().parse().unwrap();
    let parts: Vec<String> = parts[1]
    .split(":")
    .map(|x|  String::from(x.trim()))
    .collect();
    let coords = conv::to_coords(&parts[0], ",", false);
    let dims = conv::to_dims2(&parts[1], "x", false);
    (claim_id, coords, dims)
}