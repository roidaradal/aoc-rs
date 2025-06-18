use crate::aoc::{conv, io, strings, Solution};

pub fn solve() -> Solution {
    let (start, end) = data(true);

    // Part 1
    let mut count1: u32 = 0;
    for number in start..end {
        if is_valid(number) {
            count1 += 1;
        }
    }

    // Part 2 
    let mut count2: u32 = 0;
    for number in start..end {
        if is_valid2(number) {
            count2 += 1;
        }
    }

    io::solution(count1, count2)
}

fn data(full: bool) -> (usize, usize) {
    let line = io::first_line(19, 4, full);
    let p: Vec<usize> = conv::to_vec_int(&line, "-");
    (p[0], p[1])
}

fn is_valid(number: usize) -> bool {
    let x = format!("{}", number);
    if x != strings::sorted(&x) {
        return false;
    }
    strings::has_twins(&x, 0)
}

fn is_valid2(number: usize) -> bool {
    let x = format!("{}", number);
    if x != strings::sorted(&x) {
        return false;
    }
    let sizes: Vec<usize> = strings::group_chunks(&x)
        .iter()
        .map(|chunk| chunk.len())
        .collect();
    sizes.contains(&2)
}