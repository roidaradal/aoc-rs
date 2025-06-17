use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    // Part 1
    let door = data(true);
    let mut hash_gen = MD5HashGenerator{
        key : door, 
        goal: String::from("00000"),
        i   : 0,
    };
    let mut pwd = ['.'; 8];
    for i in 0..8_usize {
        let (_, hash) = hash_gen.next().unwrap();
        pwd[i] = strings::nth_char(&hash, 5);

        let tmp: String = pwd.iter().collect();
        println!("{}", tmp);
    }
    let part1: String = pwd.iter().collect();

    // Part 2
    let door = data(true);
    let mut hash_gen = MD5HashGenerator{
        key : door, 
        goal: String::from("00000"),
        i   : 0,
    };
    let mut pwd = ['.'; 8];
    loop {
        let (_, hash) = hash_gen.next().unwrap();
        let idx = strings::nth_char(&hash, 5);
        let idx: usize = idx.to_digit(16).unwrap() as usize;
        if idx >= 8 { 
            continue; 
        }
        if pwd[idx] == '.' {
            pwd[idx] = strings::nth_char(&hash, 6);
            
            let tmp: String = pwd.iter().collect();
            println!("{}", tmp);
        }


        if pwd.iter().all(|x| *x != '.') {
            break;
        }
    }
    let part2: String = pwd.iter().collect();

    io::solution(part1, part2)
}

fn data(full: bool) -> String {
    io::first_line(16, 5, full)
}

struct MD5HashGenerator {
    key : String,
    goal: String,
    i   : u32,
}

impl Iterator for MD5HashGenerator {
    type Item = (u32, String);

    fn next(&mut self) -> Option<(u32, String)> {
        loop {
            let word = format!("{}{}", self.key, self.i);
            let hash = strings::md5_hash(&word);
            if hash.starts_with(&self.goal) {
                let result = Some((self.i, hash));
                self. i += 1;
                return result;
            }
            self.i += 1;
        }
    }
}