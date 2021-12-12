use std::ops::RangeInclusive;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 4);
    let p1_count = part_1(&input);
    let p2_count = part_2(&input);
    println!("p1 {}", p1_count);
    println!("p2 {}", p2_count);
}

fn part_1(input: &str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .split("\n\n")
        .filter_map(|entry| {
            Some(true).filter(|_| fields.iter().all(|x| entry.contains(x)))
        })
        .count()
}
fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|entry| Some(true).filter(|_| entry_validator(entry)))
        .count()
}

fn entry_validator(entry: &str) -> bool {
    entry
        .split_whitespace()
        .map(|i| {
            let mut it = i.split(':');
            let field = it.next().unwrap();
            let data = it.next().unwrap();
            (field, data)
        })
        .filter(|(f, d)| data_validator(f, d))
        .count()
        == 7
}

fn data_validator(field: &str, data: &str) -> bool {
    match field {
        "byr" if data.len() == 4 => validate_range(data, 1920..=2002),
        "iyr" if data.len() == 4 => validate_range(data, 2010..=2020),
        "eyr" if data.len() == 4 => validate_range(data, 2020..=2030),
        "hgt" if data.ends_with("cm") => validate_range(data.split_at(data.len() - 2).0, 150..=193),
        "hgt" if data.ends_with("in") => validate_range(data.split_at(data.len() - 2).0, 59..=76),
        "hcl" if data.starts_with('#') => data.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => matches!(data, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" if data.len() == 9 => data.parse::<u32>().is_ok(),
        _ => false,
    }
}

fn validate_range(data: &str, range: RangeInclusive<u16>) -> bool {
    data.parse::<u16>()
        .ok()
        .filter(|x| range.contains(x))
        .is_some()
}
