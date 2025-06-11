use std::collections::{HashMap, HashSet};
use crate::aoc::{io, grid};
use crate::aoc::grid::{Delta, Coords};

pub fn solve() {
    let moves = data(true);
    let visited = walk(&moves);
    let count = visited.len();
    println!("{}", count);

    let (santa, robo) = divide_moves(moves);
    let v1 = walk(&santa);
    let v2 = walk(&robo);
    let count = v1.union(&v2).count();
    println!("{}", count);
}

fn data(full: bool) -> Vec<Delta> {
    let delta = HashMap::from([
        ('>', grid::R),
        ('<', grid::L),
        ('^', grid::U), 
        ('v', grid::D),
    ]);
    io::first_line(full)
    .chars()
    .map(|x| delta[&x])
    .collect()
}

fn walk(moves: &Vec<Delta>) -> HashSet<Coords> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let start: Coords = (0, 0);
    visited.insert(start);
    let mut curr = start;
    for d in moves.iter() {
        curr = grid::step(curr, *d);
        visited.insert(curr);
    }
    visited
}

fn divide_moves(moves: Vec<Delta>) -> (Vec<Delta>, Vec<Delta>) {
    let mut santa: Vec<Delta> = Vec::new();
    let mut robo: Vec<Delta> = Vec::new();
    for (i, d) in moves.iter().enumerate() {
        if i % 2 == 0 {
            santa.push(*d);
        } else {
            robo.push(*d);
        }
    }
    (santa, robo)
}