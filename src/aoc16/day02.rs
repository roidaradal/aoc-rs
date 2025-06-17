use std::collections::HashMap;
use crate::aoc::{grid, io, Solution};
use crate::aoc::grid::{Coords, Dims2};

pub fn solve() -> Solution {
    // Part 1
    let moves_list = data(true);
    let pad = Keypad{
        grid        : create_grid(vec!["123", "456", "789"]),
        start       : (0, 0),
        max_bounds  : (3, 3),
    };
    let part1 = solve_code(pad, moves_list);

    // Part 2
    let moves_list = data(true);
    let pad = Keypad{
        grid        : create_grid(vec!["00100", "02340", "56789", "0ABC0", "00D00"]), 
        start       : (2, 0),
        max_bounds  : (5, 5),
    };
    let part2 = solve_code(pad, moves_list);

    io::solution(part1, part2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(16, 2, full)
}

struct Keypad {
    grid        : Vec<String>,
    start       : Coords,
    max_bounds  : Dims2,
}

fn create_grid(rows: Vec<&str>) -> Vec<String> {
    rows
    .iter()
    .map(|x| x.to_string())
    .collect()
}

fn solve_code(pad: Keypad, moves_list: Vec<String>) -> String {
    let delta = HashMap::from([
        ('U', grid::U),
        ('D', grid::D),
        ('L', grid::L),
        ('R', grid::R),
    ]);
    let mut code: Vec<char> = Vec::new();
    let mut curr = pad.start;
    for moves in moves_list {
        for m in moves.chars() {
            let nxt = grid::step(curr, delta[&m]);
            if grid::inside_bounds(nxt, pad.max_bounds) && grid_char(&pad.grid, nxt) != '0' {
                curr = nxt;
            }
        }
        let x = grid_char(&pad.grid, curr);
        code.push(x);
    }
    code.into_iter().collect()
}

fn grid_char(grid: &Vec<String>, c: Coords) -> char {
    let (row, col) = grid::coords_to_index(c);
    let line = grid[row].as_bytes();
    line[col] as char
}