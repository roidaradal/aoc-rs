use std::collections::HashSet;
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let words = data(true);
    let total1: u32 = words 
        .into_iter()
        .map(common_score)
        .sum();

    // Part 2 
    let words = data(true);
    let total2: u32 = (0..words.len())
        .step_by(3)
        .into_iter()
        .map(|i| badge_score(&words[i..i+3]))
        .sum();

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(22, 3, full)
}

fn score(letter: char) -> u32 {
    let v = letter as u32;
    if 97 <= v && v <= 122 {
        v-96
    } else if 65 <= v && v <= 90 {
        v-38
    } else {
        0
    }
}

fn common_score(word: String) -> u32 {
    let mid = word.len() / 2;
    let mut chars: HashSet<char> = HashSet::new();
    for (i, letter) in word.char_indices() {
        if i < mid {
            chars.insert(letter);
        } else if chars.contains(&letter) {
            return score(letter);
        }
    }
    0
}

fn badge_score(words: &[String]) -> u32 {
    let mut common: HashSet<char> = HashSet::from_iter(words[0].chars());
    for word in words.iter().skip(1) {
        let mut uncommon: HashSet<char> = common.clone();
        for letter in word.chars() {
            if uncommon.contains(&letter) {
                uncommon.remove(&letter);
            }
        } 
        for letter in uncommon.into_iter() {
            common.remove(&letter);
        }
    }
    let common: Vec<char> = common.into_iter().collect();
    score(common[0])
}