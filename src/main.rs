use std::time::Instant;

mod aoc;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;

fn main() {
    let now = Instant::now();

    solve(18, 1);

    println!("\nTime: {:.2}s", now.elapsed().as_secs_f64())
}

fn solve(year: u8, day: u8) {
    match year {
        15 => solve15(day),
        16 => solve16(day),
        17 => solve17(day),
        18 => solve18(day),
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
        _ => println!("Invalid day"),
    }
}

fn solve16(day: u8) {
    match day {
        1 => aoc16::day01::solve(),
        2 => aoc16::day02::solve(),
        3 => aoc16::day03::solve(),
        4 => aoc16::day04::solve(),
        5 => aoc16::day05::solve(),
        _ => println!("Invalid day"),
    }
}

fn solve17(day: u8) {
    match day {
        1 => aoc17::day01::solve(),
        2 => aoc17::day02::solve(),
        3 => aoc17::day03::solve(),
        4 => aoc17::day04::solve(),
        5 => aoc17::day05::solve(),
        _ => println!("Invalid day"),
    }
}

fn solve18(day: u8) {
    match day {
        1 => aoc18::day01::solve(),
        _ => println!("Invalid day"),
    }
}