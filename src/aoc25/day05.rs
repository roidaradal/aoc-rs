use crate::aoc::{list, Solution, conv, io};


pub fn solve() -> Solution {
    let (ranges, ingredients) = data(true);
    let ranges: Vec<(u64, u64)> = list::merge_ranges(ranges);

    // Part 1 
    let count1: u64 = ingredients.iter().map(|i| {
        ranges.iter().any(|r| {
            r.0 <= *i && *i <= r.1
        }) as u64
    }).sum();

    // Part 2
    let count2: u64 = ranges.iter().map(|r| {
        r.1 - r.0 + 1
    }).sum();

    io::solution(count1, count2)
}

fn data(full: bool) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let mut part2 = false;
    for line in io::read_raw_lines(25, 5, full) {
        let line = line.trim();
        if line == "" {
            part2 = true;
        } else if part2 {
            ingredients.push(line.parse().unwrap())
        } else {
            let p: Vec<u64> = conv::to_vec_int(line, "-");
            ranges.push((p[0], p[1]))
        }
    }
    (ranges, ingredients)
}