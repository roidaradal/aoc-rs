pub type Coords = (i32, i32);
pub type GridCoords = (usize, usize);
pub type Delta  = (i32, i32);
pub type Dims2  = (u32, u32);
pub type Dims3  = (u32, u32, u32);
pub type Grid<T> = Vec<Vec<T>>;

pub const U: Delta = (-1, 0);
pub const D: Delta = (1, 0);
pub const L: Delta = (0, -1);
pub const R: Delta = (0, 1);
pub const X: Delta = (0, 0);

pub fn left_of(d: Delta) -> Delta {
    match d {
        U => L,
        L => D,
        D => R, 
        R => U,
        _ => X,
    }
}

pub fn right_of(d: Delta) -> Delta {
    match d {
        U => R, 
        R => D, 
        D => L, 
        L => U,
        _ => X,
    }
}

pub fn step(c: Coords, d: Delta) -> Coords {
    (c.0 + d.0, c.1 + d.1)
}

pub fn inside_bounds(c: Coords, max_bounds: Dims2) -> bool {
    inside_bounds_full(c, (0,0), max_bounds)
}

pub fn inside_bounds_full(c: Coords, min_bounds: Dims2, max_bounds: Dims2) -> bool {
    let (row, col) = c;
    let (min_rows, min_cols) = (min_bounds.0 as i32, min_bounds.1 as i32);
    let (num_rows, num_cols) = (max_bounds.0 as i32, max_bounds.1 as i32);
    min_rows <= row && row < num_rows && min_cols <= col && col < num_cols
}

pub fn manhattan(c1: Coords, c2: Coords) -> u32 {
    let (y1, x1) = c1;
    let (y2, x2) = c2;
    y2.abs_diff(y1) + x2.abs_diff(x1)
}

pub fn manhattan_origin(c: Coords) -> u32 {
    manhattan(c, (0,0))
}

pub fn coords_to_index(c: Coords) -> (usize, usize) {
    (c.0 as usize, c.1 as usize)
}

pub fn surround8(c: Coords) -> Vec<Coords> {
    let (y, x) = c;
    vec![
        (y-1,x-1), (y-1,x), (y-1,x+1),
        (y-0,x-1),          (y-0,x+1),
        (y+1,x-1), (y+1,x), (y+1,x+1),
    ]
}

pub fn new<T: Copy>(initial: T, rows: usize, cols: usize) -> Grid<T> {
    vec![vec![initial; cols]; rows]
}

pub fn get_delta(c1: Coords, c2: Coords) -> Delta {
    return (c2.0 - c1.0, c2.1 - c1.1)
}