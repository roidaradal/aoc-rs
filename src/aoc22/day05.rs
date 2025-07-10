use crate::aoc::{io, Solution};

type Cmd = (usize, usize, usize);

pub fn solve() -> Solution {
    // Part 1 
    let (stacks, moves) = data(true);
    let top1 = process_moves(stacks, moves, true);

    // Part 2 
    let (stacks, moves) = data(true);
    let top2 = process_moves(stacks, moves, false);

    io::solution(top1, top2)
}

fn data(full: bool) -> (Vec<Vec<char>>, Vec<Cmd>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<Cmd> = Vec::new();
    let mut stack_mode = true;
    for line in io::read_raw_lines(22, 5, full) {
        let clean = line.trim();
        if clean == "" {
            stack_mode = false;
            continue;
        }
        if stack_mode {
            if !clean.starts_with("[") {
                continue;
            }
            if stacks.len() == 0 {
                let count = line.len() / 4;
                for _ in 0..count {
                    stacks.push(Vec::new());
                }
            }
            for (i, x) in line.char_indices() {
                if i % 4 != 1 || x == ' ' {
                    continue
                }
                let idx = i / 4;
                stacks[idx].push(x);
            }
        } else {
            let p: Vec<&str> = line.split_whitespace().collect();
            let count: usize = p[1].parse().unwrap();
            let src = (p[3].parse::<usize>().unwrap()) - 1;
            let dst = (p[5].parse::<usize>().unwrap()) -1;
            moves.push((count, src, dst));
        }
    }
    (stacks, moves)
}

fn process_moves(mut stacks: Vec<Vec<char>>, moves: Vec<Cmd>, reverse: bool) -> String {
    for (count, idx1, idx2) in moves {
        stacks = transfer(stacks, count, idx1, idx2, reverse)
    }
    stacks
        .iter()
        .map(|stack| stack[0])
        .collect()
}

fn transfer(mut stacks: Vec<Vec<char>>, count: usize, idx1: usize, idx2: usize, reverse: bool) -> Vec<Vec<char>> {
    let (s1, s2) = (&stacks[idx1], &stacks[idx2]);
    let mut m = Vec::from_iter(s1[0..count].iter().copied());
    if reverse {
        m.reverse();
    }
    m.extend(s2.iter());
    stacks[idx1] = Vec::from_iter(s1[count..].iter().copied());
    stacks[idx2] = m;
    stacks
}