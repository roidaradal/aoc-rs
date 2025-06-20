use crate::aoc::{conv, io, strings, Solution};

pub fn solve() -> Solution {
    let passwords = data(true);
    let mut count1: u32 = 0;
    let mut count2: u32 = 0;
    for p in passwords {
        // Part 1
        if p.is_valid() {
            count1 += 1;
        }

        // Part 2
        if p.is_valid2() {
            count2 += 1;
        }
    }
    io::solution(count1, count2)   
}

fn data(full: bool) -> Vec<Password> {
    io::read_lines(20, 2, full)
        .into_iter()
        .map(Password::new)
        .collect()
}

struct Password {
    num1    : u32,
    num2    : u32, 
    letter  : char, 
    text    : String,
}

impl Password {
    fn new(line: String) -> Password {
        let p: Vec<&str> = line.split(":").map(str::trim).collect();
        let h: Vec<String> = p[0].split_whitespace().map(String::from).collect();
        let nums: Vec<u32> = conv::to_vec_int(&h[0], "-");
        Password{
            num1    : nums[0],
            num2    : nums[1],
            letter  : strings::nth_char(&h[1], 0),
            text    : String::from(p[1]),
        }
    }

    fn is_valid(&self) -> bool {
        let freq = strings::char_freq(&self.text, None);
        let f =  freq.get(&self.letter).copied().unwrap_or(0);
        return self.num1 <= f && f <= self.num2
    }

    fn is_valid2(&self) -> bool {
        let mut count: u32 = 0;
        let letters: Vec<char> = self.text.chars().collect();
        for idx in vec![self.num1-1, self.num2-1] {
            if letters[idx as usize] == self.letter {
                count += 1;
            }
        }
        count == 1
    }
}