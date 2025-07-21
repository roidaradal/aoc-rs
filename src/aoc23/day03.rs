use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::aoc::{io, Solution};
use crate::aoc::grid::{GridCoords, Dims2};

type Triple = (usize, usize, usize);

pub fn solve() -> Solution {
    let grid = data(true);
    
    // Part 1
    let symbols = find_symbols(&grid);
    let total1 = sum_valid_numbers(&grid, symbols);

    // Part 
    let gears = find_gears(&grid);
    let total2 = sum_gear_ratios(&grid, gears); 

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(23, 3, full)
}

fn find_symbols(grid: &Vec<String>) -> Vec<GridCoords> {
    let mut symbols: Vec<GridCoords> = Vec::new();
    let non_symbol: Vec<char> = "0123456789.".chars().collect();
    for row in 0..grid.len() {
        let line = &grid[row];
        for (col, char) in line.char_indices() {
            if !non_symbol.contains(&char) {
                symbols.push((row, col));
            }
        }

    }
    symbols
}

fn find_gears(grid: &Vec<String>) -> Vec<GridCoords> {
    let mut gears: Vec<GridCoords> = Vec::new();
    for row in 0..grid.len() {
        let line = &grid[row];
        for (col, tile) in line.char_indices() {
            if tile == '*' {
                gears.push((row, col));
            }
        }
    }
    gears
}

fn sum_valid_numbers(grid: &Vec<String>, symbols: Vec<GridCoords>) -> u32 {
    let bounds: Dims2 = (grid.len() as u32, grid[0].len() as u32);
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    for row in 0..grid.len() {
        let line = &grid[row];
        for m in number_re.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            if has_adjacent_symbol((row, start, end), &symbols, bounds) {
                let number: u32 = line[start..end].parse().unwrap();
                numbers.push(number);
            }
        }
    }
    numbers.iter().sum()
}

fn sum_gear_ratios(grid: &Vec<String>, gears: Vec<GridCoords>) -> u32 {
    let bounds: Dims2 = (grid.len() as u32, grid[0].len() as u32);
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let mut adjacent: HashMap<GridCoords, Vec<u32>> = HashMap::new();
    for row in 0..grid.len() {
        let line = &grid[row];
        for m in number_re.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            for c in get_adjacent_symbols((row, start, end), &gears, bounds) {
                let number: u32 = line[start..end].parse().unwrap();
                adjacent.entry(c).or_insert(Vec::new()).push(number);
            }
        }
    }
    let mut numbers: Vec<u32> = Vec::new();
    for (_, near) in adjacent {
        if near.len() == 2 {
            let (a, b) = (near[0], near[1]);
            numbers.push(a * b);
        }
    }
    numbers.iter().sum()
}

fn has_adjacent_symbol(triple: Triple, symbols: &Vec<GridCoords>, bounds: Dims2) -> bool {
    get_adjacent_symbols(triple, symbols, bounds).len() > 0
}

fn get_adjacent_symbols(triple: Triple, symbols: &Vec<GridCoords>, bounds: Dims2) -> Vec<GridCoords> {
    let adjacent = get_adjacent(triple, bounds);
    let mut set1: HashSet<GridCoords> = HashSet::new();
    for symbol in symbols {
        set1.insert(*symbol);
    }
    let set2: HashSet<GridCoords> = HashSet::from_iter(adjacent);
    set1.intersection(&set2).cloned().collect()
}

fn get_adjacent(triple: Triple, bounds: Dims2) -> Vec<GridCoords> {
    let (y1, x1, x2) = triple;
    let (rows, cols) = (bounds.0 as usize, bounds.1 as usize);
    let (y0, y2) = (y1 as i32 -1, y1+1);
    let (x0, x3) = (x1 as i32 -1, x2+1);
    let mut adjacent: Vec<GridCoords> = Vec::new();

    let start = if x0 >= 0 { x0 as usize } else { x1 };
    let end = if x3 <= cols { x3 } else { x2 };
    let add_above = y0 >= 0;
    let add_below = y2 < rows;
    for x in start..end {
        if add_above {
            adjacent.push((y0 as usize, x));
        }
        if add_below {
            adjacent.push((y2, x));
        }
    }
    if start != x1 {
        adjacent.push((y1, x0 as usize));
    }
    if x2 < cols {
        adjacent.push((y1, x2));
    }
    adjacent
}