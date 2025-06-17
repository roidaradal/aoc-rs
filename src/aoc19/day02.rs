use itertools::Itertools;
use crate::aoc::{conv, io, Solution};

pub fn solve() -> Solution {
    // Part 1
    let mut numbers = data(true);
    numbers[1] = 12;
    numbers[2] = 2;
    let output1 = run_program(numbers);

    // Part 2
    let mut output2: usize = 0;
    for (noun, verb) in (0..100).cartesian_product(0..100) {
        let mut numbers = data(true);
        numbers[1] = noun;
        numbers[2] = verb;
        if run_program(numbers) == 19_690_720 {
            output2 = (100 * noun) + verb;
            break;
        }
    }

    io::solution(output1, output2)
}

fn data(full: bool) -> Vec<usize> {
    let line = io::first_line(19, 2, full);
    conv::to_vec_int(&line, ",")
}

fn run_program(mut numbers: Vec<usize>) -> usize {
    let mut i: usize = 0;
    loop {
        let cmd = numbers[i];
        if cmd == 99 {
            break;
        }
        let (in1, in2, out) = (numbers[i+1], numbers[i+2], numbers[i+3]);
        if cmd == 1 {
            numbers[out] = numbers[in1] + numbers[in2];
        } else if cmd == 2 {
            numbers[out] = numbers[in1] * numbers[in2];
        }
        i += 4;
    }
    numbers[0]
}