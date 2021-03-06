use std::time::Instant;

mod aoc2018;
mod aoc2020;
mod util;

fn main() {
    let now = Instant::now();
    aoc2018::day13::run();
    println!("time spent {} us", now.elapsed().as_micros());
}
