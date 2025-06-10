use std::time::Instant;

mod aoc;
mod aoc15;

fn main() {
    let now = Instant::now();

    solve(2015, 1);

    println!("\nTime: {:.2}s", now.elapsed().as_secs_f64())
}

fn solve(year: u32, day: u32) {
    match year {
        2015 => solve15(day),
        _ => println!("Invalid year"),
    }
}

fn solve15(day: u32) {
    match day {
        1 => aoc15::day01::solve(),
        _ => println!("Invalid date"),
    }
}