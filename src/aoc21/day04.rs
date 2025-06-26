use std::collections::{HashMap, HashSet};
use crate::aoc::{conv, io, Solution};
use crate::aoc::grid::Grid;

pub fn solve() -> Solution {
    // Part 1 
    let (numbers, cards) = data(true);
    let score1 = play_bingo(numbers, cards, 1);

    // Part 2 
    let (numbers, cards) = data(true);
    let num_cards = cards.len();
    let score2 = play_bingo(numbers, cards, num_cards);

    io::solution(score1, score2)
}

fn data(full: bool) -> (Vec<u32>, Vec<Bingo>) {
    let lines = io::read_lines(21, 4, full);
    let numbers: Vec<u32> = conv::to_vec_int(&lines[0], ",");
    let mut cards: Vec<Bingo> = Vec::new();
    let mut card: Grid<u32> = Vec::new();
    for line in lines.into_iter().skip(2) {
        if line == "" {
            cards.push(Bingo::new(card));
            card = Vec::new();
        } else {
            card.push(conv::to_vec_int(&line, " "))
        }
    }
    cards.push(Bingo::new(card));
    (numbers, cards)
}

fn play_bingo(numbers: Vec<u32>, cards: Vec<Bingo>, target_count: usize) -> u32 {
    let mut cards = cards;
    let mut winners: HashSet<usize> = HashSet::new();
    let mut score: u32 = 0;
    'main_loop: for number in numbers {
        for (player, card) in cards.iter_mut().enumerate() {
            if winners.contains(&player) { continue }

            card.mark(number);
            if card.has_won() {
                winners.insert(player);
            }
            if winners.len() == target_count {
                score = number * card.score();
                break 'main_loop;
            }
        }
    }
    score
}

type GridCoords = (usize, usize);

struct Bingo {
    card    : Grid<u32>,
    rows    : usize, 
    cols    : usize, 
    marked  : HashMap<GridCoords, bool>,
    lookup  : HashMap<u32, GridCoords>,
}

impl Bingo {
    fn new(card: Grid<u32>) -> Bingo {
        let (rows, cols) = (card.len(), card[0].len());
        let mut b = Bingo {
            card    : card, 
            rows    : rows,
            cols    : cols,
            marked  : HashMap::new(),
            lookup  : HashMap::new(),
        };
        for row in 0..rows {
            for col in 0..cols {
                let pt = (row, col);
                let x = b.card[row][col];
                b.lookup.insert(x, pt);
                b.marked.insert(pt, false);
            }
        }
        b
    }

    fn mark(&mut self, number: u32) {
        if self.lookup.contains_key(&number) {
            let c = self.lookup[&number];
            self.marked.insert(c, true);
        }
    }

    fn has_won(&self) -> bool {
        for row in 0..self.rows {
            if (0..self.cols).all(|col| self.marked[&(row,col)]) {
                return true;
            }
        }
        for col in 0..self.cols {
            if (0..self.rows).all(|row| self.marked[&(row,col)]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.marked.keys()
            .filter(|c| !self.marked[c])
            .map(|c| {
                let (row, col) = (c.0, c.1);
                self.card[row][col]
            }).sum()
    }
}