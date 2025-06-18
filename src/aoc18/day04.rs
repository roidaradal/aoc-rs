use std::collections::HashMap;
use crate::aoc::{io, Solution};

type SleepMap = HashMap<u32, HashMap<u32, u32>>;

pub fn solve() -> Solution {
    // Part 1
    let logs = data(true);
    let sleep = process_logs(logs);
    let guard1 =  max_asleep(sleep);

    // Part 2
    let logs = data(true);
    let sleep = process_logs(logs);
    let guard2 = max_freq_asleep(sleep);

    io::solution(guard1, guard2)
}

const OFF: u32 = 0;
const ON: u32 = 1;
const GUARD: u32 = 2;

fn data(full: bool) -> Vec<(u32, u32)> {
    let mut lines = io::read_lines(18, 4, full);
    lines.sort();
    lines
        .iter()
        .map(|x| {
            let p: Vec<&str> = x.split("]").map(|x| x.trim()).collect();
            let h: Vec<&str> = p[0].split(":").collect();
            let minute: u32 = h[1].parse().unwrap();
            let tail = p[1];
            if tail.contains("asleep") {
                return (ON, minute);
            } else if tail.contains("wakes") {
                return (OFF, minute);
            } else {
                let t: Vec<&str> = tail.split_whitespace().collect();
                let id: u32 = t[1].strip_prefix("#").unwrap().parse().unwrap();
                return (GUARD, id);
            }
        })
        .collect()
}

fn process_logs(logs: Vec<(u32,u32)>) -> SleepMap {
    let limit = logs.len();
    let mut i = 0;
    let mut guard: u32 = 0;
    let mut sleep: SleepMap = HashMap::new();
    while i < limit {
        let (cmd, x) = logs[i];
        if cmd == GUARD {
            guard = x;
            i += 1;
        } else if cmd == ON {
            let end = logs[i+1].1;
            for m in x..end {
                *sleep.entry(guard).or_insert(HashMap::new()).entry(m).or_insert(0) += 1;
            }
            i += 2;
        }
    }
    sleep
}

fn max_asleep(sleep: SleepMap) -> u32 {
    let guard = sleep
        .iter()
        .map(|e| {
            let (guard, s) = e;
            let total: u32 = s.values().sum();
            (total, guard)
        })
        .max()
        .unwrap().1;

    let minute = sleep.get(guard)
        .unwrap()
        .iter()
        .map(|e| {
            let (m, count) = e;
            (count, m)
        })
        .max()
        .unwrap().1;

    guard * minute
}

fn max_freq_asleep(sleep: SleepMap) -> u32 {
    let guards: Vec<[&u32; 3]> = sleep 
        .iter()
        .map(|e| {
            let (guard, s) = e;
            let (count, minute) = s.iter().map(|e| {
                let (k, v) = e;
                (v,k)
            }).max().unwrap();
            [count, minute, guard]
        }).collect();
    
    let max_guard = guards.iter().max().unwrap();
    let (minute, guard) = (max_guard[1], max_guard[2]);
    
    minute * guard
}