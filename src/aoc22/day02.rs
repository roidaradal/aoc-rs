use std::collections::HashMap;
use crate::aoc::{io, Solution};

const R: u32 = 1;
const P: u32 = 2;
const S: u32 = 3;
const L: u32 = 0;
const D: u32 = 3;
const W: u32 = 6;

pub fn solve() -> Solution {
    // Part 1 
    let map: HashMap<&str, u32> = HashMap::from([
        ("A", R), ("B", P), ("C", S),
        ("X", R), ("Y", P), ("Z", S),
    ]);
    let games = data(true, map);
    let total1: u32 = games
        .into_iter()
        .map(compute_score)
        .sum();

    // Part 2
    let map: HashMap<&str, u32> = HashMap::from([
        ("A", R), ("B", P), ("C", S),
        ("X", L), ("Y", D), ("Z", W),
    ]);
    let games = data(true, map);
    let total2: u32 = games 
        .into_iter()
        .map(coerce_score)
        .sum();

    io::solution(total1, total2)
}

fn data(full: bool, map: HashMap<&str, u32>) -> Vec<(u32, u32)> {
    io::read_lines(22, 2, full)
        .into_iter()
        .map(|line| {
            let p: Vec<&str> = line.split_whitespace().collect();
            (map[p[0]], map[p[1]])
        })
        .collect()
}

fn wins_over(hand: u32) -> u32 {
    match hand {
        R => S,
        P => R, 
        S => P,
        _ => 0,
    }
}

fn loses_to(hand: u32) -> u32 {
    match hand {
        S => R, 
        R => P, 
        P => S, 
        _ => 0,
    }
}

fn compute_score(game: (u32, u32)) -> u32 {
    let (opp, you) = game;
    let mut score: u32 = you;
    if opp == you {
        score += D;
    } else if wins_over(you) == opp {
        score += W;
    }
    score
}

fn coerce_score(game: (u32, u32)) -> u32 {
    let (opp, outcome) = game;
    let you = match outcome {
        W => loses_to(opp),
        L => wins_over(opp),
        D => opp,
        _ => 0,
    };
    compute_score((opp, you))
}