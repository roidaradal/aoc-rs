use itertools::Itertools;
use crate::aoc::{io, conv};

pub fn solve() {
    let numbers_list = data(true);
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for numbers in numbers_list {
        let min = numbers.iter().min().unwrap();
        let max = numbers.iter().max().unwrap();
        total1 += max - min;

        for mut pair in numbers.iter().combinations(2) {
            pair.sort();
            let (a, b) = (pair[0], pair[1]);
            if b % a == 0 {
                total2 += b / a;
                break;
            }
        }
    }
    println!("{}", total1);
    println!("{}", total2);
}

fn data(full: bool) -> Vec<Vec<u32>> {
    io::read_lines(full)
    .iter()
    .map(|x| conv::to_vec_int(x, " "))
    .collect()
}
