pub type Dims3 = (u32, u32, u32);

pub fn new_dims3(line: &String, sep: &str) -> Dims3 {
    let parts: Vec<&str> = line.split(sep).collect();
    let p0: u32 = parts.get(0).unwrap().parse().unwrap();
    let p1: u32 = parts.get(1).unwrap().parse().unwrap();
    let p2: u32 = parts.get(2).unwrap().parse().unwrap();
    (p0, p1, p2)
}
