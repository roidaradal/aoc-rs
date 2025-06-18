use crate::aoc::{conv, io, Solution};
use super::intcode;

pub fn solve() -> Solution {
    // Part 1
    let numbers = data(true);
    let output1 = run_program(numbers, 1);

    // Part 2 
    let numbers = data(true);
    let output2 = run_program(numbers, 5);

    io::solution(output1, output2)
}

fn data(full: bool) -> Vec<i32> {
    let line = io::first_line(19, 5, full);
    conv::to_vec_int(&line, ",")
}

fn run_program(mut numbers: Vec<i32>, start: i32) -> i32 {
    let mut output = 0;
    let mut i: usize = 0;
    loop {
        let (head, tail) = command_parts(numbers[i]);
        let cmd: u8 = tail.parse().unwrap();
        if cmd == 99 {
            break;
        }

        if cmd == 1 || cmd == 2 || cmd == 7 || cmd == 8 {
            let (in1, in2, out) = (numbers[i+1], numbers[i+2], numbers[i+3] as usize);
            let m = intcode::modes(head, 3);
            let (m1, m2) = (m[0], m[1]);
            let a = intcode::param(in1, m1, &numbers);
            let b = intcode::param(in2, m2, &numbers);
            match cmd {
                1 => numbers[out] = a + b,
                2 => numbers[out] = a * b,
                7 => numbers[out] = if a < b { 1 } else { 0 },
                8 => numbers[out] = if a == b { 1 } else { 0 },
                _ => (),
            }
            i += 4
        } else if cmd == 3 {
            let idx = numbers[i+1] as usize;
            numbers[idx] = start;
            i += 2;
        } else if cmd == 4 {
            let m = intcode::modes(head, 1)[0];
            let out = intcode::param(numbers[i+1], m, &numbers);
            if out != 0 {
                output = out;
            }
            i += 2;
        } else if cmd == 5 || cmd == 6 {
            let (p1, p2) = (numbers[i+1], numbers[i+2]);
            let m = intcode::modes(head, 2);
            let (m1, m2) = (m[0], m[1]);
            let is_zero = intcode::param(p1, m1, &numbers) == 0;
            let do_jump = if cmd == 6 { is_zero } else { !is_zero };
            if do_jump {
                i = intcode::param(p2, m2, &numbers) as usize;
            } else {
                i += 3;
            }
        }
    }
    output
}

fn command_parts(number: i32) -> (String, String) {
    let word = format!("{}", number);
    if word.len() <= 2 {
        (String::from(""), word)
    } else {
        let chars: Vec<char> = word.chars().collect();
        let n = chars.len();
        let head: String = chars[0..n-2].iter().collect();
        let tail: String = chars[n-2..].iter().collect();
        (head, tail) 
    }
}