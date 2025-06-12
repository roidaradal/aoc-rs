pub fn to_dims3(line: &String, sep: &str) -> (u32, u32, u32) {
    let p = to_vec_int(line, sep);
    (p[0], p[1], p[2])
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

fn to_vec_int_space(line: &String) -> Vec<u32> {
    line
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}