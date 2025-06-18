use std::collections::HashMap;
use crate::aoc::{grid, io, Solution};
use crate::aoc::grid::{Coords, Delta};

pub fn solve() -> Solution {
    // Part 1
    let wires = data(true);
    let closest1 = crossing_points(wires)
        .into_keys()
        .map(|c| grid::manhattan_origin(c))
        .min()
        .unwrap();

    // Part 2 
    let wires = data(true);
    let closest2 = crossing_points(wires)
        .into_values()
        .min()
        .unwrap();

    io::solution(closest1, closest2)
}

type WireMoves = Vec<(Delta, i32)>;
type WiresList = Vec<WireMoves>;

fn data(full: bool) -> WiresList {
    let delta = HashMap::from([
        ('U', grid::U),
        ('D', grid::D),
        ('L', grid::L), 
        ('R', grid::R),
    ]);
    io::read_lines(19, 3, full)
        .iter()
        .map(|x| {
            x.split(",")
                .map(|m| {
                    let chars: Vec<char> = m.chars().collect();
                    let d = delta[&chars[0]];
                    let steps: String = chars[1..].iter().collect();
                    let steps: i32 = steps.parse().unwrap();
                    (d, steps)
                })
                .collect()
        })
        .collect()
}

fn crossing_points(wires: WiresList) -> HashMap<Coords, u32> {
    let mut steps: HashMap<Coords, Vec<u32>> = HashMap::new();
    for moves in wires {
        let visited = wire(moves);
        for (c, x) in visited {
            steps.entry(c).or_insert(Vec::new()).push(x);
        }
    }
    let mut cross: HashMap<Coords, u32> = HashMap::new();
    for (c, s) in  steps {
        if s.len() > 1 {
            cross.insert(c, s.iter().sum());
        }
    }
    cross
}

fn wire(moves: WireMoves) -> HashMap<Coords, u32> {
    let mut visited: HashMap<Coords, u32> = HashMap::new();
    let mut c: Coords = (0, 0);
    let mut i: u32 = 0;
    for (d, steps) in moves {
        for _ in 0..steps {
            c = grid::step(c, d);
            i += 1;
            if !visited.contains_key(&c) {
                visited.insert(c, i);
            }
        }
    }
    visited
}