use crate::util::get_puzzle_input;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\d+)-(\d+)\s+(\w+):\s+(\w+)").unwrap();
}

pub fn run() {
    let input = get_puzzle_input(2020, 2);
    let mut p1_count = 0;
    let mut p2_count = 0;
    input.lines().for_each(|line| {
        let matches = REG.captures(line).unwrap();
        let min = matches.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let max = matches.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let letter = matches.get(3).unwrap().as_str().parse::<char>().unwrap();
        let input = matches.get(4).unwrap().as_str();
        if p1_check_password(input, letter, min, max) {
            p1_count += 1;
        }
        if p2_check_password(input, letter, min, max) {
            p2_count += 1;
        }
    });
    println!("p1 {}", p1_count);
    println!("p2 {}", p2_count);
}

fn p1_check_password(input: &str, letter: char, min: u8, max: u8) -> bool {
    let c = input.chars().filter(|x| *x == letter).count() as u8;
    c >= min && c <= max
}

fn p2_check_password(input: &str, letter: char, min: u8, max: u8) -> bool {
    let in_min = input
        .chars()
        .nth(min as usize - 1)
        .filter(|x| *x == letter)
        .is_some();
    let in_max = input
        .chars()
        .nth(max as usize - 1)
        .filter(|x| *x == letter)
        .is_some();
    in_min != in_max
}
