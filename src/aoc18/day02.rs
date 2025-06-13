use itertools::Itertools;
use crate::aoc::{io, strings};

pub fn solve() {
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
    println!("{}", count2 * count3);

    for pair in words.iter().combinations(2) {
        let (word1, word2) = (pair[0], pair[1]);
        let diff = str_diff(word1, word2);
        if diff.len() != 1 { continue; }

        let idx = diff[0];
        println!("{}{}", &word1[..idx], &word1[idx+1..]);
        break;
    }
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(full)
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