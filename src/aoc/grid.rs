pub type Coords = (i32, i32);
pub type Delta  = (i32, i32);
pub type Dims3  = (u32, u32, u32);

pub const U: Delta = (-1, 0);
pub const D: Delta = (1, 0);
pub const L: Delta = (0, -1);
pub const R: Delta = (0, 1);

pub fn mov(c: Coords, d: Delta) -> Coords {
    (c.0 + d.0, c.1 + d.1)
}

pub fn to_dims3(line: &String, sep: &str) -> Dims3 {
    let mut p0: u32 = 0;
    let mut p1: u32 = 0;
    let mut p2: u32 = 0;
    let parts: Vec<&str> = line.split(sep).collect();
    for i in 0..3 {
        let x: u32 = parts.get(i).unwrap().parse().unwrap();
        match i {
            0 => p0 = x,
            1 => p1 = x,
            2 => p2 = x,
            _ => continue,
        }
    }
    (p0, p1, p2)
}