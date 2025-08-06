use crate::aoc::{conv, io, Solution};

pub fn solve() -> Solution {
    let numbers_list = data(true);

    let mut count1 = 0;
    let mut count2 = 0;
    for numbers in numbers_list {
        // Part 1 
        if is_safe(&numbers) {
            count1 += 1;
        }

        // Part 2 
        if is_safe2(&numbers) {
            count2 += 1;
        }
    }

    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<Vec<i32>> {
    io::read_lines(24, 2, full)
        .into_iter()
        .map(|line| {
            conv::to_vec_int(&line, " ")
        })
        .collect()
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = (1..numbers.len()).map(|i| {
        numbers[i] - numbers[i-1]
    }).collect();
    let safe_inc = diffs.iter().all(|d| {
        1 <= *d && *d <= 3
    });
    let safe_dec = diffs.iter().all(|d| {
        -3 <= *d && *d <= -1
    });
    safe_inc || safe_dec
}

fn is_safe2(numbers: &Vec<i32>) -> bool {
    if is_safe(numbers) {
        return true;
    }
    for idx in 0..numbers.len() {
        let mut numbers2: Vec<i32> = Vec::new();
        numbers2.extend_from_slice(&numbers[..idx]);
        numbers2.extend_from_slice(&numbers[idx+1..]);
        if is_safe(&numbers2) {
            return true;
        }
    }
    false
}