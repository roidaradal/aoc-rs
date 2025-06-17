use itertools::Itertools;
use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    // Part 1
    let words = data(true);
    let mut count2: u32 = 0;
    let mut count3: u32 = 0;
    for word in words.iter() {
        let freq: Vec<u32> = strings::char_freq(word, None).into_values().collect();
        if freq.contains(&2) {
            count2 += 1;
        }
        if freq.contains(&3) {
            count3 += 1;
        }
    }
    let part1 = count2 * count3;

    // Part 2
    let mut part2: String = String::new();
    for pair in words.iter().combinations(2) {
        let (word1, word2) = (pair[0], pair[1]);
        let diff = str_diff(word1, word2);
        if diff.len() == 1 {
            let idx = diff[0];
            part2 = format!("{}{}", &word1[..idx], &word1[idx+1..]);
            break;
        }
    }

    io::solution(part1, part2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(18, 2, full)
}

fn str_diff(word1: &str, word2: &str) -> Vec<usize> {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut diff: Vec<usize> = Vec::new();
    for i in 0..word1.len() {
        if word1[i] != word2[i] {
            diff.push(i);
        }
    }
    diff
}