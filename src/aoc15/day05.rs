use std::collections::HashMap;
use crate::aoc::{io, strings};

pub fn solve() {
    let words = data(true);
    let mut count1: u32 = 0;
    let mut count2: u32 = 0;
    for word in words {
        if is_nice(&word) {
            count1 += 1;
        }
        if is_nice2(&word) {
            count2 += 1;
        }
    }
    println!("{}", count1);
    println!("{}", count2);
}

fn data(full: bool) -> Vec<String> {
    return io::read_lines(full);
}

const INVALIDS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice(word: &String) -> bool {
    let has_invalid = INVALIDS.iter().any(|invalid| word.find(invalid).is_some());
    if has_invalid {
        return false;
    }

    if !strings::has_twins(word, 0) {
        return false;
    }

    let freq = strings::char_freq(word, None);
    let mut num_vowels: u32 = 0;
    for vowel in VOWELS{
        if let Some(count) = freq.get(&vowel) {
            num_vowels += count;
        }
    }
    num_vowels >= 3
}

fn is_nice2(word: &String) -> bool {
    if !strings::has_twins(word, 1) {
        return false;
    }

    let pairs = substring_positions(word, 2);
    for idxs in pairs.values() {
        if idxs.len() >= 3 {
            return true;
        } else if idxs.len() == 2 {
            if idxs[0].abs_diff(idxs[1]) >= 2 {
                return true;
            }
        }
    }
    false
}

fn substring_positions(word: &String, length: usize) -> HashMap<String, Vec<usize>> {
    let mut group: HashMap<String, Vec<usize>> = HashMap::new();
    let limit = word.len() - (length-1);
    for i in 0..limit {
        let sub = word[i..i+length].to_string();
        group.entry(sub).or_insert(Vec::new()).push(i);
    }
    group
}