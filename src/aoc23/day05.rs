use std::collections::{HashMap, VecDeque};
use crate::aoc::{conv, io, Solution};

type Int2 = (i64, i64);
type Int3 = (i64, i64, i64);

pub fn solve() -> Solution {
    let (seeds, maps) = data(true);
    let seed_chain: Vec<&str> = vec!("seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location");

    // Part 1 
    let location1 = apply_map_chain(&seeds, &maps, &seed_chain);

    // Part 2 
    let location2 = apply_map_range_chain(&seeds, &maps, &seed_chain);

    io::solution(location1, location2)
}

fn data(full: bool) -> (Vec<i64>, HashMap<String, Vec<Int3>>) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: HashMap<String, Vec<Int3>> = HashMap::new();
    let mut key: String = String::from("");
    for text in io::read_lines(23, 5, full) {
        if text == "" {
            continue
        } else if text.starts_with("seeds:") {
            let parts: Vec<&str> = text.split(":").collect();
            seeds = conv::to_vec_int(parts[1], " ");
        } else if text.ends_with("map:") {
            let parts: Vec<&str> = text.split_whitespace().collect();
            key = String::from(parts[0]);
        } else {
            let parts: Vec<i64> = conv::to_vec_int(&text, " ");
            let (dst, src, count) = (parts[0], parts[1], parts[2]);
            maps.entry(key.clone()).or_insert(Vec::new()).push((src, dst, count))
        }
    }
    (seeds, maps)
}

fn apply_map_chain(seeds: &Vec<i64>, maps: &HashMap<String, Vec<Int3>>, seed_chain: &Vec<&str>) -> i64 {
    let mut current: Vec<i64> = seeds.clone();
    let limit = seed_chain.len() - 1;
    for i in 0..limit {
        let key = format!("{}-to-{}", seed_chain[i], seed_chain[i+1]);
        current = plant_translate(current, &maps[&key]);
    }
    current.into_iter().min().unwrap()
}

fn plant_translate(numbers: Vec<i64>, triples: &Vec<Int3>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for x in numbers {
        let mut y = x;
        for t in triples {
            let (src, dst, count) = (t.0, t.1, t.2);
            if src <= x && x < src+count {
                y = dst + (x-src);
                break;
            }
        }
        result.push(y);
    }
    result
}

fn seed_ranges(seeds: &Vec<i64>) -> Vec<Int2> {
    let mut ranges: Vec<Int2> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        ranges.push((seeds[i], seeds[i] + seeds[i+1]-1));
    }
    ranges   
}

fn map_ranges(maps: &HashMap<String, Vec<Int3>>) -> HashMap<String, Vec<Int3>> {
    let mut m: HashMap<String, Vec<Int3>> = HashMap::new();
    for (key, triples) in maps {
        for t in triples {
            let (src, dst, count) = (t.0, t.1, t.2);
            m.entry(key.clone()).or_insert(Vec::new()).push((src, src+count-1, dst-src));
        }
    }
    m
}

fn apply_map_range_chain(seeds: &Vec<i64>, maps: &HashMap<String, Vec<Int3>>, seed_chain: &Vec<&str>) -> i64 {
    let mut curr_ranges = VecDeque::from(seed_ranges(seeds));
    let range_map = map_ranges(maps);
    let limit = seed_chain.len() - 1;
    for i in 0..limit {
        let key = format!("{}-to-{}", seed_chain[i], seed_chain[i+1]);
        let mut next_ranges: VecDeque<Int2> = VecDeque::new();
        while curr_ranges.len() > 0 {
            let curr_range = curr_ranges.pop_front().unwrap();
            let mut found = false;
            for t in range_map[&key].iter() {
                let (start, end, diff) = (t.0, t.1, t.2);
                let rule_range: Int2 = (start, end);
                if is_inside(rule_range, curr_range) {
                    let (first, last) = curr_range;
                    next_ranges.push_back((first + diff, last + diff));
                    found = true;
                    break;
                }
                let (matched, extra) = find_intersection(rule_range, curr_range);
                if matched.is_some() && extra.is_some() {
                    let (first, last) = matched.unwrap();
                    next_ranges.push_back((first + diff, last + diff));
                    curr_ranges.push_back(extra.unwrap());
                    found = true;
                    break;
                }
            }
            if !found {
                next_ranges.push_back(curr_range);
            }
        }
        curr_ranges = next_ranges;
    }
    curr_ranges.into_iter().map(|curr_range| {
        curr_range.0
    }).min().unwrap()
}

fn is_inside(rule_range: Int2, curr_range: Int2) -> bool {
    let (min_val, max_val) = rule_range;
    let (start, end) = curr_range;
    min_val <= start && start <= end && end <= max_val
}

fn find_intersection(rule_range: Int2, curr_range: Int2) -> (Option<Int2>, Option<Int2>) {
    let (min_val, max_val) = rule_range;
    let (start, end) = curr_range;
    if min_val <= start && start <= max_val {
        (Some((start, max_val)), Some((max_val+1, end)))
    }else if min_val <= end && end <= max_val {
        (Some((min_val, end)), Some((start, min_val-1)))
    } else {
        (None, None)
    }

}