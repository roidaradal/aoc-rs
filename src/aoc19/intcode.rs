pub fn modes(cmd: String, count: usize) -> Vec<u8> {
    let mut m: Vec<u8> = Vec::new();
    for _ in 0..count {
        m.push(0);
    }
    let mut i: usize = 0;
    for x in cmd.chars().rev() {
        m[i] = String::from(x).parse().unwrap();
        i += 1;
    }
    m
}

pub fn param(x: i32, mode: u8, numbers: &Vec<i32>) -> i32 {
    if mode == 0 {
        numbers[x as usize]
    } else {
        x
    }
}