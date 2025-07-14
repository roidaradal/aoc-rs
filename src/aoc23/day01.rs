use std::collections::HashMap;

use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    let lines = data(true);

    let mut total1 = 0;
    let mut total2 = 0;
    for line in lines {
        // Part 1 
        total1 += extract_digits(&line);

        // Part 2 
        total2 += extract_number(&line);
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(23, 1, full)
}

fn extract_digits(line: &String) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for x in line.chars() {
        let digit = parse_digit(x);
        if digit > 0 {
            (first, last) = update_digits(first, digit);
        }
    }
    (first * 10) + last
}

fn extract_number(line: &String) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for (i, x) in line.char_indices() {
        let digit = parse_digit(x);
        if digit > 0 {
            (first, last) = update_digits(first, digit);
            continue;
        }
        let digit = parse_number(&line[i..]);
        if digit > 0 {
            (first, last) = update_digits(first, digit)
        }
    }

    (first * 10) + last
}

fn parse_digit(x: char) -> u32 {
    let v = x as u32;
    if 48 <= v && v <= 57 {
        v-48
    } else {
        0
    }
}

fn parse_number(text: &str) -> u32 {
    let number_words: HashMap<&str, u32> = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ]);
    for (word, number) in number_words {
        if text.starts_with(word) {
            return number;
        }
    }
    0
}

fn update_digits(first: u32, digit: u32) -> (u32, u32) {
    let first2 = if first == 0 {
        digit
    } else {
        first
    };
    (first2, digit)
}