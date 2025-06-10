use std::collections::HashMap;

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
    let mut freq: HashMap<char, u32> = HashMap::new();
    for char in word.chars() {
        if let Some(skip) = &skip {
            if skip.contains(&char) {
                continue;
            }
        }
        *freq.entry(char).or_insert(0) += 1;
    }
    freq
}