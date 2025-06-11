pub type Coords = (i32, i32);
pub type Delta  = (i32, i32);
pub type Dims3  = (u32, u32, u32);
pub type Int2   = (i32, i32);

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

pub fn to_dims3(line: &String, sep: &str) -> Dims3 {
    let p: Vec<u32> = line 
    .split(sep)
    .map(|x| x.parse().unwrap())
    .collect();
    (p[0], p[1], p[2])
}

pub fn manhattan(c1: Coords, c2: Coords) -> u32 {
    c2.0.abs_diff(c1.0) + c2.1.abs_diff(c1.1)
}

pub fn manhattan_origin(c: Coords) -> u32 {
    manhattan(c, (0,0))
}