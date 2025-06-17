use std::collections::{HashMap, HashSet};
use crate::aoc::{grid, io, Solution};
use crate::aoc::grid::{Delta, Coords};

pub fn solve() -> Solution {
    // Part 1
    let moves = data(true);
    let visited = walk(moves);
    let count1 = visited.len();

    // Part 2
    let moves = data(true);
    let (santa, robo) = divide_moves(moves);
    let v1 = walk(santa);
    let v2 = walk(robo);
    let count2 = v1.union(&v2).count();

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<Delta> {
    let delta = HashMap::from([
        ('>', grid::R),
        ('<', grid::L),
        ('^', grid::U), 
        ('v', grid::D),
    ]);
    io::first_line(15, 3, full)
    .chars()
    .map(|x| delta[&x])
    .collect()
}

fn walk(moves: Vec<Delta>) -> HashSet<Coords> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let start: Coords = (0, 0);
    visited.insert(start);
    let mut curr = start;
    for d in moves {
        curr = grid::step(curr, d);
        visited.insert(curr);
    }
    visited
}

fn divide_moves(moves: Vec<Delta>) -> (Vec<Delta>, Vec<Delta>) {
    let mut santa: Vec<Delta> = Vec::new();
    let mut robo: Vec<Delta> = Vec::new();
    let limit = moves.len();
    for i in (0..limit).step_by(2) {
        santa.push(moves[i]);
        robo.push(moves[i+1]);
    }
    (santa, robo)
}