use std::{fmt::Debug, str::FromStr};

pub fn to_vec_int<T: FromStr>(line: &str, sep: &str) -> Vec<T> where <T as FromStr>::Err: Debug{
    if sep == " " {
        line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    } else {
        line
            .split(sep)
            .map(|x| x.trim().parse().unwrap())
            .collect()
    }
}

pub fn to_dims3(line: &str, sep: &str) -> (u32, u32, u32) {
    let p = to_vec_int(line, sep);
    (p[0], p[1], p[2])
}

pub fn to_dims2(line: &str, sep: &str, row_col: bool) -> (u32, u32) {
    let p = to_vec_int(line, sep);
    if row_col {
        (p[0], p[1])
    } else {
        (p[1], p[0])
    }
}

pub fn to_int2(line: &str, sep: &str) -> (i32, i32) {
    let p = to_vec_int(line, sep);
    (p[0], p[1])
}

pub fn to_coords(line: &str, sep: &str, row_col: bool) -> (i32, i32) {
    let c = to_vec_int(line, sep);
    if row_col {
        (c[0], c[1])
    } else {
        (c[1], c[0])
    }
}
