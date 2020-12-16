use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref RULES_RE: Regex = Regex::new(r"((\d+)-(\d+))").unwrap();
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 16);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

#[inline]
fn parse(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let mut it = input.split("\n\n");
    let rules_str = it.next().unwrap();
    let my_ticket_str = it.next().unwrap().lines().skip(1).next().unwrap();
    let other_tickets_it = it.next().unwrap().lines().skip(1);
    let rules = Rules::new(rules_str);
    let my_ticket = Ticket::new(my_ticket_str);
    let other_tickets = parse_multiple_tickets(other_tickets_it);
    (rules, my_ticket, other_tickets)
}

struct Rules(Vec<Vec<RangeInclusive<usize>>>);

impl Rules {
    #[inline]
    fn new(rules_str: &str) -> Self {
        Self(
            rules_str
                .lines()
                .map(|line| {
                    RULES_RE
                        .captures_iter(line)
                        .map(|c| {
                            let start = c.get(2).unwrap().as_str().parse().unwrap();
                            let end = c.get(3).unwrap().as_str().parse().unwrap();
                            start..=end
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

struct Ticket(Vec<usize>);

impl Ticket {
    #[inline]
    fn new(ticket_str: &str) -> Self {
        Self(ticket_str.split(",").map(|x| x.parse().unwrap()).collect())
    }
}
#[inline]
fn parse_multiple_tickets<'a>(multiple_ticket_it: impl Iterator<Item = &'a str>) -> Vec<Ticket> {
    multiple_ticket_it.map(|line| Ticket::new(line)).collect()
}

fn part1(input: &str) -> usize {
    let (rules, _my_ticket, other_tickets) = parse(&input);
    let mut sum = 0;
    for t in other_tickets {
        for field in t.0 {
            if !rules.0.iter().flatten().any(|range| range.contains(&field)) {
                sum += field;
            }
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let (rules, my_ticket, other_tickets) = parse(&input);
    // filter invalid tickets first
    let valid_tickets = other_tickets.iter().filter(|t| {
        let valid =
            t.0.iter()
                .all(|field| rules.0.iter().flatten().any(|range| range.contains(&field)));
        valid
    });
    // construct possible ticket matches
    let mut field_vec = vec![(0..rules.0.len()).collect::<HashSet<_>>(); rules.0.len()];
    for t in valid_tickets {
        for (field_id, field) in t.0.iter().enumerate() {
            for (rule_id, rule) in rules.0.iter().enumerate() {
                if !rule.iter().any(|range| range.contains(&field)) {
                    field_vec[field_id].remove(&rule_id);
                }
            }
        }
    }
    // no need for complex backtrack search, there is always one clear choice.
    let mut field_to_rule_mapping = HashMap::new();
    while field_to_rule_mapping.len() < rules.0.len() {
        for id in 0..rules.0.len() {
            if field_vec[id].len() == 1 {
                let rule_id = *field_vec[id].iter().next().unwrap();
                field_to_rule_mapping.insert(id, rule_id);
                // propagate
                for i in field_vec.iter_mut() {
                    i.remove(&rule_id);
                }
            }
        }
    }
    my_ticket
        .0
        .iter()
        .enumerate()
        .fold(1, |mut acc, (i, field)| {
            if *field_to_rule_mapping.get(&i).unwrap() < 6 {
                acc *= field;
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_16_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 71);
    }
}
