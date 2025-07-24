use std::collections::{HashMap, HashSet};
use crate::aoc::{conv, io, Solution};

type Game = (HashSet<u32>, HashSet<u32>);

pub fn solve() -> Solution {
    let games = data(true);

    let mut total1: u32 = 0;
    let limit = games.len();
    let mut count: HashMap<usize, usize> = HashMap::new();
    for i in 0..limit {
        count.insert(i, 1);
    }
    for (i, game) in games.into_iter().enumerate() {
        let (winners, numbers) = game;
        let common = winners.intersection(&numbers).count();

        // Part 1 
        if common > 0 {
            total1 += 2_u32.pow((common-1) as u32);
        }

        // Part 2 
        for j in 0..common {
            let k = i + j + 1;
            if k < limit {
                *count.get_mut(&k).unwrap() += count[&i];
            }
        }
    }

    let total2: usize = count.values().sum();

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<Game> {
    io::read_lines(23, 4, full)
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let parts: Vec<&str> = parts[1].split("|").collect();
            let winners: Vec<u32> = conv::to_vec_int(parts[0], " ");
            let numbers: Vec<u32> = conv::to_vec_int(parts[1], " ");
            let winners: HashSet<u32> = HashSet::from_iter(winners);
            let numbers: HashSet<u32> = HashSet::from_iter(numbers);
            (winners, numbers)
        }).collect()
}
