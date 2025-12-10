use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use chrono::{Datelike, Utc};
use curl::easy::{Easy2, Handler, WriteError};
use dotenv::dotenv;

const USER_AGENT: &str = "github.com/gokberkkocak/adventofcode using libcurl";

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

/// Returns the puzzle input
///
pub fn get_puzzle_input(year: u32, day: u8) -> String {
    let filename = format!("inputs/{}_{}.in", year, day);
    if Path::new(&filename).exists() {
        read_file(&filename)
    } else {
        dotenv().ok();
        if let Ok(session) = env::var("AOC_SESSION") {
            let mut easy = Easy2::new(Collector(Vec::new()));
            easy.get(true).unwrap();
            let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            easy.url(&input_url).unwrap();
            easy.cookie(&format!("session={}", session)).unwrap();
            easy.useragent(USER_AGENT).unwrap();
            easy.perform().unwrap();
            let contents = easy.get_ref();
            write_to_file(&contents.0, &filename);
            return String::from_utf8_lossy(&contents.0).trim_end().to_string();
        }
        panic!("Session credentials are missing");
    }
}

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file into the string");
    contents.trim_end().to_string()
}

fn write_to_file(contents: &[u8], filepath: &str) {
    let file = File::create(filepath).expect("Unable to create file");
    let mut buffered_writer = BufWriter::new(file);
    buffered_writer
        .write_all(contents)
        .expect("Unable to write to file");
}

pub fn get_latest_year_and_day() -> (i32, u32) {
    let now = Utc::now();
    let last_aoc_day = if now.year() > 2024 { 12 } else { 25 };
    match now.month() {
        12 => (now.year(), now.day().min(last_aoc_day)),
        _ => (now.year() - 1, last_aoc_day),
    }
}
