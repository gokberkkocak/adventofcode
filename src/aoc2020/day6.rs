use std::collections::HashSet;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 6);
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("{}", p1);
    println!("{}", p2);
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| {
            x.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>()
}

fn full_answer() -> HashSet<char> {
    ('a'..='z').collect()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|e| e.chars().collect::<HashSet<char>>())
                .fold(full_answer(), |acc, e| {
                    acc.intersection(&e).cloned().collect()
                })
                .len()
        })
        .sum::<usize>()
}
