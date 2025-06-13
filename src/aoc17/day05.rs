use crate::aoc::io;

pub fn solve() {
    let mut jumps = data(true);
    let count = count_jumps(&mut jumps, false);
    println!("{}", count);

    let mut jumps = data(true);
    let count = count_jumps(&mut jumps, true);
    println!("{}", count);
}

fn data(full: bool) -> Vec<i32> {
    io::read_lines(full)
    .iter()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn count_jumps(jumps: &mut Vec<i32>, clip: bool) -> u32 {
    let limit = jumps.len() as i32;
    let mut i = 0;
    let mut count: u32 = 0;
    while 0 <= i && i < limit {
        let jump = jumps[i as usize];
        let increment = if clip && jump >= 3 { -1 } else { 1 };
        jumps[i as usize] = jump + increment;
        i += jump;
        count += 1;
    }
    count
}