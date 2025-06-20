use std::collections::HashMap;
use regex::Regex;
use crate::aoc::{io, Solution};

pub fn solve() -> Solution {
    let passports = data(true);
    let mut count1 = 0;
    let mut count2 = 0;
    for p in passports {
        // Part 1 
        if p.has_all_keys() {
            count1 += 1;
        }

        // Part 2
        if p.is_valid() {
            count2 += 1;
        }
    }
    io::solution(count1, count2)
}

fn data(full: bool) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut p = Passport::new();
    for line in io::read_lines(20, 4, full) {
        if line == "" {
            passports.push(p);
            p = Passport::new();
        } else {
            for pair in line.split_whitespace() {
                let parts: Vec<&str> = pair.split(":").collect();
                let (key, value) = (parts[0].to_string(), parts[1].to_string());
                p.add_info(key, value);
            }
        }
    }
    passports.push(p);
    passports
}

struct Passport {
    info: HashMap<String, String>
}

impl Passport {
    fn new() -> Passport {
        Passport { info: HashMap::new() }
    }

    fn add_info(&mut self, key: String, value: String) {
        self.info.insert(key, value);
    }

    fn has_all_keys(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|k| self.info.contains_key(k))
    }

    fn is_valid(&self) -> bool {
        let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
        let ecl_options = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let mut ok = 0;
        for (k, v) in self.info.iter() {
            let k = k.as_str();
            match k {
                "byr" => {
                    let byr: u32 = v.parse().unwrap();
                    if 1920 <= byr && byr <= 2002 {
                        ok += 1;
                    }
                },
                "iyr" => {
                    let iyr: u32 = v.parse().unwrap();
                    if 2010 <= iyr && iyr <= 2020 {
                        ok += 1;
                    }
                },
                "eyr" => {
                    let eyr: u32 = v.parse().unwrap();
                    if 2020 <= eyr && eyr <= 2030 {
                        ok += 1;
                    }
                },
                "hgt" => {
                    let n = v.len();
                    let hgt: u32 = if n > 2 {
                        v[..n-2].parse().unwrap()
                    } else {
                        0
                    };
                    if v.ends_with("cm") && 150 <= hgt && hgt <= 193  {
                        ok += 1;
                    }
                    if v.ends_with("in") && 59 <= hgt && hgt <= 76 {
                        ok += 1;
                    }
                },
                "hcl" => {
                    if hcl_re.is_match(v) {
                        ok += 1;
                    }
                },
                "ecl" => {
                    if ecl_options.contains(&v.as_str()) {
                        ok += 1;
                    }
                },
                "pid" => { 
                    if pid_re.is_match(v) {
                        ok += 1;
                    }
                },
                _ => (),
            }
        }
        ok == 7
    }
}