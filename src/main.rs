use std::time::Instant;

use structopt::StructOpt;
use year_lib::AOCYear;

mod aoc2018;
mod aoc2020;
mod aoc2021;
mod aoc2022;
mod util;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "AoC runner.")]
struct Opt {
    /// Run all solutions for a given year.
    #[structopt(short, long)]
    all: bool,

    /// Which year to run (default: last available aoc year)
    #[structopt(short, long)]
    year: Option<i32>,

    /// Which day to run (default: last available aoc day)
    #[structopt(short, long)]
    day: Option<u32>,
}

fn main() {
    let opt = Opt::from_args();
    let (year, day) = return_year_and_day(&opt);
    let aoc_year = return_aoc_year(year);
    let now = Instant::now();
    match &opt.all {
        true => aoc_year.run_all(),
        false => aoc_year.run_day(day),
    }
    println!("time spent {} us", now.elapsed().as_micros());
}

fn return_year_and_day(opt: &Opt) -> (i32, u32) {
    let (latest_year, latest_day) = util::get_latest_year_and_day();
    let year = opt.year.unwrap_or(latest_year);
    let day = opt.day.unwrap_or(latest_day);
    (year, day)
}

fn return_aoc_year(year: i32) -> Box<dyn AOCYear> {
    match year {
        2018 => aoc2018::AOC2018::new(),
        2020 => aoc2020::AOC2020::new(),
        2021 => aoc2021::AOC2021::new(),
        2022 => aoc2022::AOC2022::new(),
        _ => unimplemented!("year {}", year),
    }
}
