use std::collections::HashMap;
use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    // Part 1 
    let binary_numbers = data(true);
    let mid = binary_numbers.len() / 2;
    let num_digits = binary_numbers[0].len();
    let mut count: HashMap<usize, usize> = HashMap::new();
    for code in binary_numbers {
        for (i, bit) in code.char_indices() {
            if bit == '1' {
                *count.entry(i).or_insert(0) += 1;
            }
        }
    }
    let mut g: Vec<char> = Vec::new();
    let mut e: Vec<char> = Vec::new();
    for i in 0..num_digits {
        if count[&i] > mid {
            g.push('1');
            e.push('0');
        } else {
            g.push('0');
            e.push('1');
        }
    }
    let g: String = g.iter().collect();
    let e: String = e.iter().collect();
    let g = parse_binary(&g);
    let e = parse_binary(&e);
    let part1 = g * e;
    

    // Part 2
    let binary_numbers = data(true);
    let oxy = filter_max(&binary_numbers);
    let co2 = filter_min(&binary_numbers);
    let part2 = oxy * co2;

    io::solution(part1, part2)
}

fn data(full: bool) -> Vec<String> {
    io::read_lines(21, 3, full)
}

fn parse_binary(number: &str) -> i32 {
    i32::from_str_radix(number, 2).unwrap()
}

fn count_index(numbers: &Vec<String>, index: usize) -> usize {
    let valid: Vec<&String> = numbers
        .iter()
        .filter(|code| strings::nth_char(code, index) == '1')
        .collect();
    valid.len()
}

fn filter_max(numbers: &Vec<String>) -> i32 {
    let mut numbers: Vec<String> = numbers.iter().map(String::from).collect();
    let bit_length = numbers[0].len();
    for i in 0..bit_length {
        let c1 = count_index(&numbers, i);
        let c0 = numbers.len() - c1;
        let max_bit = if c1 >= c0 { '1' } else { '0' };
        numbers = numbers.into_iter().filter(|code| {
            strings::nth_char(code, i) == max_bit
        }).collect();
        if numbers.len() == 1 { break }
    }
    parse_binary(&numbers[0])
}

fn filter_min(numbers: &Vec<String>) -> i32 {
    let mut numbers: Vec<String> = numbers.iter().map(String::from).collect();
    let bit_length = numbers[0].len();
    for i in 0..bit_length {
        let c1 = count_index(&numbers, i);
        let c0 = numbers.len() - c1;
        let min_bit = if c0 <= c1 { '0' } else { '1' };
        numbers = numbers.into_iter().filter(|code| {
            strings::nth_char(code, i) == min_bit
        }).collect();
        if numbers.len() == 1 { break }
    }
    parse_binary(&numbers[0])
}
