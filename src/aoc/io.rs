use std::{collections::HashMap, env, fmt::Display, fs};
use crate::aoc::Solution;

fn root_dir() -> String {
    env::var("AOC_DATA_DIR").unwrap_or(String::from("../aoc-data"))
}

pub fn read_lines(year: u8, day: u8, full: bool) -> Vec<String> {
    let folder = if full { format!("20{}",year) } else { String::from("test") };
    let path = format!("{}/{}/{}{:0>2}.txt", root_dir(), folder, year, day);
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_raw_lines(year: u8, day: u8, full: bool) -> Vec<String> {
    let folder = if full { format!("20{}",year) } else { String::from("test") };
    let path = format!("{}/{}/{}{:0>2}.txt", root_dir(), folder, year, day);
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect()
}

pub fn first_line(year: u8, day: u8, full: bool) -> String {
    read_lines(year, day, full)[0].to_string()
    // lines.first().unwrap().to_string()
    // lines.into_iter().nth(0).unwrap()
}

pub fn get_solution(year: u8, day: u8) -> Solution {
    let path = format!("{}/solutions/all.csv", root_dir());
    let mut solutions: HashMap<String, Solution> = HashMap::new();
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .for_each(|line| {
            let p: Vec<&str> = line.split("|").collect();
            let k = format!("{}{}", p[0], p[1]);
            let v = (String::from(p[2]), String::from(p[3]));
            solutions.insert(k, v);
        });
    let key = format!("{}{:0>2}", year, day);
    solutions.get(&key).unwrap().clone()
}

pub fn solution<T: Display, V: Display>(part1: T, part2: V) -> Solution {
    let part1 = format!("{}", part1);
    let part2 = format!("{}", part2);
    (part1, part2)
}