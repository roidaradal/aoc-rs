use std::cmp;
use crate::aoc::{io, Int3, Solution};

pub fn solve() -> Solution {
    let games = data(true);

    let mut total1 = 0;
    let mut total2 = 0;
    for game in games {
        // Part 1
        if game.is_valid() {
            total1 += game.id;
        }

        // Part 2
        total2 += game.power();
    }

    io::solution(total1, total2)
}

fn data(full: bool) -> Vec<MarbleGame> {
    io::read_lines(23, 2, full)
        .into_iter()
        .map(|line| {
            let p: Vec<&str> = line.split(":").collect();
            let head: Vec<&str> = p[0].split_whitespace().collect();
            let id = head[1].parse().unwrap();
            let mut game = MarbleGame::new(id);
            for draw in p[1].split(";") {
                game.add_draw(draw);
            }
            game
        })
        .collect()
}

struct MarbleGame {
    id: i32, 
    draws: Vec<Int3>,
}

impl MarbleGame {
    fn new(id: i32) -> MarbleGame {
        MarbleGame{
            id: id, 
            draws: Vec::new(),
        }
    }

    fn add_draw(&mut self, line: &str) {
        let mut r = 0;
        let mut g = 0; 
        let mut b = 0;
        for part in line.split(",") {
            let p: Vec<&str> = part.split_whitespace().collect();
            let number = p[0].parse().unwrap();
            match p[1] {
                "red"   => r = number,
                "green" => g = number, 
                "blue"  => b = number,
                _       => (),
            }
        }
        self.draws.push((r, g, b));
    }

    fn is_valid(&self) -> bool {
        for (r, g, b ) in &self.draws {
            if *r > 12 || *g > 13 || *b > 14 {
                return false;
            }
        }
        true
    }

    fn power(&self) -> i32 {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for (r, g, b) in &self.draws {
            max_r = cmp::max(max_r, *r);
            max_g = cmp::max(max_g, *g);
            max_b = cmp::max(max_b, *b);
        }
        max_r * max_g * max_b
    }
}