use crate::aoc::{Solution, io};
use crate::aoc::grid::{inside_bounds, surround8};

pub fn solve() -> Solution {
    let mut grid = data(true);
    let bounds = (grid.len() as u32, grid[0].len() as u32);
    let mut total1 = 0;
    let mut total2  = 0;
    loop {
        let mut grid2: Vec<Vec<bool>> = Vec::new();
        let mut count = 0;
        for (row, line) in grid.iter().enumerate() {
            let mut line2: Vec<bool> = line.clone();
            for (col, paper) in line.iter().enumerate() {
                if !paper { continue }
                let paper_neighbors = surround8((row as i32, col as i32))
                    .into_iter().filter(|(r,c)| {
                        inside_bounds((*r,*c), bounds) && grid[*r as usize][*c as usize]
                    }).count();
                if paper_neighbors < 4 {
                    line2[col] = false;
                    count += 1;
                }
            }
            grid2.push(line2);
        }
        grid = grid2;
        if total1 == 0 {
            total1 = count;
        }
        total2 += count;
        if count == 0 { break }
    }
    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<Vec<bool>> {
    io::read_lines(25, 4, full)
        .iter()
        .map(|line| {
            line.chars().map(|x| x == '@').collect()            
        })
        .collect()
}