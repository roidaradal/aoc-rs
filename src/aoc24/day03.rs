use regex::Regex;
use crate::aoc::{conv::to_vec_int, io, Solution};

type Cmd = (String, usize);
type Region = (usize, usize);

pub fn solve() -> Solution {
    let text = data(true);
    let mul_re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut commands: Vec<Cmd> = Vec::new();

    // Part 1 
    let mut total1 = 0;
    for m in mul_re.find_iter(&text) {
        let (start, end) = (m.start(), m.end());
        let cmd = String::from(&text[start..end]);
        total1 += exec_command(cmd);
        let cmd = String::from(&text[start..end]);
        commands.push((cmd, start))
    }

    // Part 2
    let off_re = Regex::new(r"don't\(\)").unwrap();
    let on_re = Regex::new(r"do\(\)").unwrap();
    let mut regions: Vec<Region> = Vec::new();
    regions.push((0,1));
    for m in off_re.find_iter(&text) {
        regions.push((m.start(), 0));
    }
    for m in on_re.find_iter(&text) {
        regions.push((m.start(), 1));
    }
    regions.sort();

    let mut ignore: Vec<Region> = Vec::new();
    let mut off_start: Option<usize> = None;
    for (start, flag) in regions {
        if flag == 0 && off_start.is_none() { 
            off_start = Some(start);
        } else if flag == 1 && off_start.is_some() {
            ignore.push((off_start.unwrap(), start-1));
            off_start = None;
        }
    }
    if off_start.is_some() {
        ignore.push((off_start.unwrap(), text.len() -1));
    }

    let mut total2 = 0;
    for (cmd, start) in commands {
        let invalid = ignore.iter().any(|p| {
            let (a, b) = (p.0, p.1);
            a <= start && start <= b
        });
        if !invalid {
            total2 += exec_command(cmd);
        }
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> String {
    io::read_lines(24, 3, full).join("")
}

fn exec_command(cmd: String) -> i32 {
    let cmd = cmd.strip_prefix("mul(").unwrap();
    let cmd = cmd.strip_suffix(")").unwrap();
    let p: Vec<i32> = to_vec_int(cmd, ",");
    p[0] * p[1]
}