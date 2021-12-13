use std::collections::HashMap;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"\d+").unwrap();
    static ref TERMINAL_REG: Regex = Regex::new(r"\w+").unwrap();
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 19);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p1 {}", p2);
}

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    One(Vec<u8>),
    Or(Vec<u8>, Vec<u8>),
}

fn parse(input: &str) -> (HashMap<u8, Rule>, &str) {
    let mut it = input.split("\n\n");
    let rules_str = it.next().unwrap();
    let words_str = it.next().unwrap();
    let mut map = HashMap::new();
    for r_line in rules_str.lines() {
        let mut r_it = r_line.split(':');
        let parent = r_it.next().unwrap().parse::<u8>().unwrap();
        let children_str = r_it.next().unwrap();
        if !children_str.contains('\"') {
            let children_it = children_str.split('|');
            let rules = children_it
                .map(|child| {
                    REG.captures_iter(child)
                        .map(|x| x.get(0).unwrap().as_str().parse::<u8>().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<_>>>();
            if rules.len() == 1 {
                map.insert(parent, Rule::One(rules[0].clone()));
            } else {
                map.insert(parent, Rule::Or(rules[0].clone(), rules[1].clone()));
            }
        } else {
            let terminal_rules = TERMINAL_REG
                .captures_iter(children_str)
                .map(|x| x.get(0).unwrap().as_str().chars().next().unwrap())
                .collect::<Vec<_>>();
            debug_assert!(terminal_rules.len() == 1);
            map.insert(parent, Rule::Char(terminal_rules[0]));
        }
    }
    (map, words_str)
}

fn solve<'a>(word: &'a str, rule: u8, rules_map: &HashMap<u8, Rule>) -> Vec<&'a str> {
    fn solve_inner<'a>(
        word: &'a str,
        rules: &[u8],
        rules_map: &HashMap<u8, Rule>,
    ) -> Vec<&'a str> {
        let mut words = vec![word];
        for rule in rules {
            let mut new_words = Vec::new();
            for word in words {
                new_words.append(&mut solve(word, *rule, rules_map))
            }
            words = new_words;
        }
        words
    }

    if word.is_empty() {
        return vec![];
    }
    match rules_map.get(&rule).unwrap() {
        Rule::Char(c) => word
            .strip_prefix(*c)
            .map(|m| vec![m])
            .unwrap_or_else(Vec::new),
        Rule::One(rules) => solve_inner(word, rules, rules_map),
        Rule::Or(rules_1, rules_2) => {
            let mut words = Vec::new();
            words.append(&mut solve_inner(word, rules_1, rules_map));
            words.append(&mut solve_inner(word, rules_2, rules_map));
            words
        }
    }
}

fn part1(input: &str) -> usize {
    let (rules, words) = parse(input);
    words
        .lines()
        .flat_map(|word| solve(word, 0, &rules))
        .filter(|&x| x.is_empty())
        .count()
}

fn alter_input(input: &str) -> String {
    input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31")
}

fn part2(input: &str) -> usize {
    let altered_input = alter_input(input);
    let (rules, words) = parse(&altered_input);
    words
        .lines()
        .flat_map(|word| solve(word, 0, &rules))
        .filter(|&x| x.is_empty())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_19_test_1.in");
        let p1 = part1(&input);
        assert_eq!(p1, 2);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_19_test_2.in");
        let p2 = part2(&input);
        assert_eq!(p2, 12);
    }
}
