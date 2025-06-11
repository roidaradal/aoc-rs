use std::time::Instant;

mod aoc;
mod aoc15;
mod aoc16;

fn main() {
    let now = Instant::now();

    solve(16, 1);

    println!("\nTime: {:.2}s", now.elapsed().as_secs_f64())
}

fn solve(year: u8, day: u8) {
    match year {
        15 => solve15(day),
        16 => solve16(day),
        _ => println!("Invalid year"),
    }
}

fn solve15(day: u8) {
    match day {
        1 => aoc15::day01::solve(),
        2 => aoc15::day02::solve(),
        3 => aoc15::day03::solve(),
        4 => aoc15::day04::solve(),
        5 => aoc15::day05::solve(),
        _ => println!("Invalid date"),
    }
}

fn solve16(day: u8) {
    match day {
        1 => aoc16::day01::solve(),
        _ => println!("Invalid date"),
    }
}