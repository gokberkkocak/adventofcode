use std::collections::{HashMap, HashSet};

use crate::util::get_puzzle_input;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref BIG_BAG_RE: Regex = Regex::new(r"^(\w+ \w+) bag").unwrap();
    static ref SMALL_BAG_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
}

pub fn run() {
    let input = get_puzzle_input(2020, 7);
    let mut big_to_small = HashMap::new();
    let mut small_to_big = HashMap::new();
    input.lines().for_each(|line| {
        let outer_matches = BIG_BAG_RE.captures(line).unwrap();
        let big_bag = outer_matches.get(1).unwrap().as_str();
        let inner_matches = SMALL_BAG_RE.captures_iter(line);
        inner_matches.for_each(|i| {
            let quantity = i.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let small_bag = i.get(2).unwrap().as_str();
            small_to_big
                .entry(small_bag)
                .or_insert_with(Vec::new)
                .push((big_bag, quantity));
            big_to_small
                .entry(big_bag)
                .or_insert_with(Vec::new)
                .push((small_bag, quantity));
        });
    });
    let p1 = part1(&small_to_big);
    let p2 = part2(&big_to_small);
    println!("p1 {}", p1);
    println!("p2 {}", p2);
}

fn part1<'a>(small_to_big: &HashMap<&'a str, Vec<(&'a str, usize)>>) -> usize {
    get_all_bigger_containers("shiny gold", small_to_big).len()
}

fn get_all_bigger_containers<'a>(
    small_bag: &'a str,
    container_map: &HashMap<&'a str, Vec<(&'a str, usize)>>,
) -> HashSet<&'a str> {
    match container_map.get(small_bag) {
        Some(t) => t
            .iter()
            .flat_map(|(x, _q)| {
                let mut h = get_all_bigger_containers(x, container_map);
                h.insert(x);
                h
            })
            .collect::<HashSet<&str>>(),
        None => HashSet::new(),
    }
}

fn part2<'a>(big_to_small: &HashMap<&'a str, Vec<(&'a str, usize)>>) -> usize {
    get_nb_smaller_containers("shiny gold", big_to_small) - 1
}

fn get_nb_smaller_containers<'a>(
    big_bag: &'a str,
    container_map: &HashMap<&'a str, Vec<(&'a str, usize)>>,
) -> usize {
    match container_map.get(big_bag) {
        Some(t) => {
            t.iter()
                .map(|(x, q)| q * get_nb_smaller_containers(x, container_map))
                .sum::<usize>()
                + 1
        }
        None => 1,
    }
}
