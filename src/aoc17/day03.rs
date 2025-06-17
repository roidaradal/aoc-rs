use std::collections::HashMap;
use crate::aoc::{grid, io, Solution};
use crate::aoc::grid::Coords;

pub fn solve() -> Solution {
    // Part 1
    let x = data(true);
    let c = spiral_coords(x);
    let dist = grid::manhattan_origin(c);

    // Part 2
    let goal = x;
    let mut spiral: HashMap<Coords, u32> = HashMap::new();
    spiral.insert((0,0), 1);
    let mut values: Vec<u32> = vec![0, 1];
    let mut x: u32 = 2;
    let mut value: u32 = 0;
    while value <= goal {
        let curr = spiral_coords(x);
        let near: Vec<Coords> = grid::surround8(curr)
        .into_iter()
        .filter(|c| spiral.contains_key(c))
        .collect();
        value = near
        .iter()
        .map(|c| {
            let idx = (*spiral.get(c).unwrap()) as usize;
            values[idx]
        })
        .sum();
        values.push(value);
        spiral.insert(curr, x);
        x += 1;
    }

    io::solution(dist, value)
}

fn data(full: bool) -> u32 {
    io::first_line(17, 3, full).parse().unwrap()
}

fn spiral_coords(x: u32) -> Coords {
    let mut layer = spiral_layer(x);
    let (side, offset) = spiral_offset(x, layer);
    if side == 'T' || side == 'L' {
        layer = -layer;
    }
    if side == 'T' || side == 'B' {
        (layer, offset)
    } else {
        (offset, layer)
    }
}

fn spiral_layer(x: u32) -> i32 {
    let x = x as f32;
    let mut dims = x.sqrt().ceil() as i32;
    if dims % 2 == 0 {
        dims += 1;
    }
    (dims-1) / 2
}

const SIDE: [char; 4] = ['B','L','T','R'];
fn spiral_offset(x: u32, layer: i32) -> (char, i32) {
    let corners = spiral_corners(layer);
    for (i, pair) in corners.windows(2).enumerate() {
        let (c2, c1) = (pair[0], pair[1]);
        if !(c2 >= x && x >= c1) {
            continue;
        }

        let mid = (c1 + c2) / 2;
        let mut offset = (mid as i32) - (x as i32);
        if i < 2 {
            offset = -offset;
        }
        return (SIDE[i], offset);
    }
    ('C', 0)
}

fn spiral_corners(layer: i32) -> Vec<u32> {
    let mut corners: Vec<u32> = Vec::new();
    if layer == 0 {
        return corners;
    }
    let dims = ((layer * 2) + 1) as u32;
    let step = dims - 1;
    let square = dims.pow(2);
    corners.push(square);
    for i in 0..4 {
        corners.push(corners[i as usize]-step);
    }
    corners
}