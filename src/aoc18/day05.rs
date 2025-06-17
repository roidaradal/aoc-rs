use std::cmp;
use itertools::Itertools;
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    // Part 1 
    let word = data(true);
    let word = fully_compress(word);
    let length = word.len();

    // Part 2 
    let word = data(true);
    let mut chars: Vec<char> = word.chars().map(|x| x.to_ascii_lowercase()).unique().collect();
    chars.sort();
    let num_chars = chars.len();
    let mut min_length = usize::MAX;
    for (i, skip_char) in chars.iter().enumerate() {
        let word2: String = word.chars()
        .filter(|x| x.to_ascii_lowercase() != *skip_char)
        .collect();
        let word2 = fully_compress(word2);
        let word_length = word2.len();
        min_length = cmp::min(min_length, word_length);
        println!("{:0>2} / {:0>2} - {} - {}", i+1, num_chars, skip_char, word_length);
    }

    io::solution(length, min_length)
}

fn data(full: bool) -> String {
    io::first_line(18, 5, full)
}

fn fully_compress(mut word: String) -> String {
    let mut ok = true;
    while ok {
        (word, ok) = compress(word);
    }
    word
}

fn compress(word: String) -> (String, bool) {
    let letters: Vec<char> = word.chars().collect();
    let limit = word.len() - 1;
    for i in 0..limit {
        let (x, y) = (letters[i], letters[i+1]);
        if x != y && x.to_ascii_lowercase() == y.to_ascii_lowercase() {
            let word = format!("{}{}", &word[..i], &word[i+2..]);
            return (word, true);
        }
    }
    (word, false)
}