use std::collections::HashMap;
use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    let phrases = data(true);
    let mut count1: u32 = 0;
    let mut count2: u32 = 0;
    for phrase in phrases.iter() {
        // Part 1
        if is_valid(phrase, false) {
            count1 += 1;
        }

        // Part 2
        if is_valid(phrase, true) {
            count2 += 1;
        }
    }
    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<Vec<String>> {
    io::read_lines(17, 4, full)
    .iter()
    .map(|line| line.split_whitespace().map(String::from).collect())
    .collect()
}

fn is_valid(phrase: &Vec<String>, sort_word: bool) -> bool {
    let mut freq: HashMap<String, u32> = HashMap::new();
    for word in phrase {
        let key = if sort_word {
            strings::sorted(word)
        } else {
            String::from(word)
        };
        *freq.entry(key).or_insert(0) += 1;
    }
    freq.values().all(|count| *count == 1)
}