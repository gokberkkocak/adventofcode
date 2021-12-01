use std::time::Instant;

use aoc::Year;
use structopt::StructOpt;

mod aoc;
mod aoc2018;
mod aoc2020;
mod aoc2021;
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
    let year_obj = return_year_dyn(year);
    let now = Instant::now();
    match &opt.all {
        true => year_obj.run_all(),
        false => year_obj.run_day(day),
    }
    println!("time spent {} us", now.elapsed().as_micros());
}

fn return_year_and_day(opt: &Opt) -> (i32, u32) {
    let (default_year, default_day) = util::get_latest_aoc_date();
    let year = opt.year.unwrap_or(default_year);
    let day = opt.day.unwrap_or(default_day);
    (year, day)
}

fn return_year_dyn(year: i32) -> Box<dyn Year> {
    match year {
        2018 => Box::new(aoc2018::Year2018::default()),
        2020 => Box::new(aoc2020::Year2020::default()),
        2021 => Box::new(aoc2021::Year2021::default()),
        _ => panic!("Year {} not implemented", year),
    }
}
