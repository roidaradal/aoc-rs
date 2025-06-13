pub fn to_dims3(line: &String, sep: &str) -> (u32, u32, u32) {
    let p = to_vec_int(line, sep);
    (p[0], p[1], p[2])
}

pub fn to_dims2(line: &String, sep: &str, row_col: bool) -> (u32, u32) {
    let p = to_vec_int(line, sep);
    if row_col {
        (p[0], p[1])
    } else {
        (p[1], p[0])
    }
}

pub fn to_vec_int(line: &String, sep: &str) -> Vec<u32> {
    if sep == " " {
        return to_vec_int_space(line);
    }
    line
    .split(sep)
    .map(|x| x.trim().parse().unwrap())
    .collect()
}

pub fn to_coords(line: &String, sep: &str, row_col: bool) -> (i32, i32) {
    let c: Vec<i32> = line
    .split(sep)
    .map(|x| x.trim().parse().unwrap())
    .collect();
    if row_col {
        (c[0], c[1])
    } else {
        (c[1], c[0])
    }
}

fn to_vec_int_space(line: &String) -> Vec<u32> {
    line
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}