use std::{collections::HashMap, iter, str};
use md5;

pub fn md5_hash(word: &String) -> String {
    let digest = md5::compute(word);
    format!("{:x}", digest)
}

pub fn has_twins(word: &String, gap: usize) -> bool{
    let back = gap + 1;
    let chars = word.as_bytes();
    for i in back..chars.len() {
        if chars[i] == chars[i-back] {
            return true;
        }
    }
    false
}

pub fn char_freq(word: &String, skip: Option<Vec<char>>) -> HashMap<char,u32> {
    let has_skip = skip.is_some();
    let skip = skip.unwrap_or(Vec::new());
    let mut freq: HashMap<char, u32> = HashMap::new();
    for char in word.chars() {
        if has_skip && skip.contains(&char) {
            continue;
        }
        *freq.entry(char).or_insert(0) += 1;
    }
    freq
}

pub fn repeat_string(word: &str, repeat: usize) -> String {
    iter::repeat(word).take(repeat).collect()
}

pub fn repeat_char(letter: char, repeat: usize) -> String {
    (0..repeat).map(|_| letter).collect()
}

pub fn nth_char(word: &str, n: usize) -> char {
    word.chars().nth(n).unwrap()
}

pub fn next_char(c: char) -> char {
    char::from_u32(c as u32 + 1).unwrap()
}

pub fn sorted(word: &str) -> String {
    let mut letters: Vec<char> = word.chars().collect();
    letters.sort();
    letters.iter().collect()
}

pub fn group_chunks(word: &str) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let letters: Vec<char> = word.chars().collect();
    let mut curr = letters[0];
    let mut count = 1;
    for i in 1..letters.len() {
        let letter = letters[i];
        if letter == curr {
            count += 1;
        } else {
            chunks.push(repeat_char(curr, count));
            (curr, count) = (letter, 1);
        }
    }
    chunks.push(repeat_char(curr,count));
    chunks
}