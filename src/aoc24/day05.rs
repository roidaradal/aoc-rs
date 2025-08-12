use std::collections::{HashMap, HashSet};
use crate::aoc::{conv, io, Int2, Solution};

type Rules = HashMap<i32, HashSet<i32>>;

pub fn solve() -> Solution {
    let (rules, pages) = data(true);

    let mut total1 = 0;
    let mut total2 = 0;
    for page in pages {
        // Part 1 
        if is_valid_page(&page, &rules) {
            let mid = page.len() / 2;
            total1 += page[mid];
        }

        // Part 2
        total2 += correct_order_mid(page, &rules);
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> (Rules, Vec<Vec<i32>>) {
    let mut rules: Vec<Int2> = Vec::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();
    let mut part2 = false;
    for line in io::read_lines(24, 5, full) {
        if part2 {
            pages.push(conv::to_vec_int(&line, ","));
        } else if line == "" {
            part2 = true;
        } else {
            rules.push(conv::to_int2(&line, "|"))
        }
    }
    let mut book: Rules = HashMap::new();
    for (before, after) in rules {
        book.entry(after).or_insert(HashSet::new()).insert(before);
    }
    (book, pages)
}

fn is_valid_page(page: &Vec<i32>, rules: &Rules) -> bool {
    for i in 0..(page.len()-1) {
        let after: HashSet<i32> = HashSet::from_iter(page[i+1..].iter().cloned());
        let blacklist = &rules[&page[i]];
        let common = after.intersection(blacklist).count();
        if common > 0 {
            return false;
        }
    }
    true
}

fn correct_order_mid(mut page: Vec<i32>, rules: &Rules) -> i32 {
    let mut valid = true;
    let limit = page.len() - 1;
    let mut idx = 0;
    while idx < limit {
        let curr = page[idx];
        let after: HashSet<i32> = HashSet::from_iter(page[idx+1..].iter().cloned());
        let blacklist = &rules[&curr];
        let common: Vec<&i32> = after.intersection(blacklist).collect();
        if common.len() == 0 {
            idx += 1;
        } else {
            valid = false;
            let indexes: Vec<usize> = common.into_iter().map(|x| {
                page.iter().position(|y| x == y).unwrap()
            }).collect();
            let insert = indexes.iter().max().unwrap() + 1;
            page[idx] = 0;
            let mut page2: Vec<i32> = Vec::new();
            page2.extend_from_slice(&page[..insert]);
            page2.push(curr);
            page2.extend_from_slice(&page[insert..]);
            page = page2.into_iter().filter(|x| *x != 0).collect();
        }
    }
    let mid = page.len() / 2;
    if valid { 0 } else { page[mid] }
}