use std::env;
use std::time::Instant;
use dotenv::dotenv;
use crate::aoc::Solution;

mod aoc;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
mod aoc19;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("Usage: cargo run -- yydd (test)");
        return;
    }

    let yydd = &args[0];
    let year: u8 = yydd[..2].parse().unwrap();
    let day: u8 = yydd[2..].parse().unwrap();
    let test_mode = args.len() >= 2 && args[1] == "test";
    
    dotenv().ok();
    let (sol1, sol2) = aoc::io::get_solution(year, day);
    
    let now = Instant::now();

    let (ans1, ans2) = solve(year, day);
    if test_mode {
        if ans1 == sol1 {
            println!("OK1: {}", sol1);
        } else {
            println!("Part1: Exp vs Got:\n{}\n{}", sol1, ans1);
        }
        if ans2 == sol2 {
            println!("OK2: {}", sol2);
        } else {
            println!("Part2: Exp vs Got:\n{}\n{}", sol2, ans2);
        }
    } else {
        println!("{}", ans1);
        println!("{}", ans2);
    }

    println!("\nTime: {:.2}s", now.elapsed().as_secs_f64())
}

fn solve(year: u8, day: u8) -> Solution {
    match year {
        15 => solve15(day),
        16 => solve16(day),
        17 => solve17(day),
        18 => solve18(day),
        19 => solve19(day),
        _ => panic!("Invalid year")
    }
}

fn solve15(day: u8) -> Solution {
    match day {
        1 => aoc15::day01::solve(),
        2 => aoc15::day02::solve(),
        3 => aoc15::day03::solve(),
        4 => aoc15::day04::solve(),
        5 => aoc15::day05::solve(),
        _ => panic!("Invalid day"),
    }
}

fn solve16(day: u8) -> Solution {
    match day {
        1 => aoc16::day01::solve(),
        2 => aoc16::day02::solve(),
        3 => aoc16::day03::solve(),
        4 => aoc16::day04::solve(),
        5 => aoc16::day05::solve(),
        _ => panic!("Invalid day"),
    }
}

fn solve17(day: u8) -> Solution {
    match day {
        1 => aoc17::day01::solve(),
        2 => aoc17::day02::solve(),
        3 => aoc17::day03::solve(),
        4 => aoc17::day04::solve(),
        5 => aoc17::day05::solve(),
        _ => panic!("Invalid day"),
    }
}

fn solve18(day: u8) -> Solution {
    match day {
        1 => aoc18::day01::solve(),
        2 => aoc18::day02::solve(),
        3 => aoc18::day03::solve(),
        4 => aoc18::day04::solve(),
        5 => aoc18::day05::solve(),
        _ => panic!("Invalid day"),
    }
}

fn solve19(day: u8) -> Solution {
    match day {
        1 => aoc19::day01::solve(),
        2 => aoc19::day02::solve(),
        3 => aoc19::day03::solve(),
        4 => aoc19::day04::solve(),
        5 => aoc19::day05::solve(),
        _ => panic!("Invalid day"),
    }
}