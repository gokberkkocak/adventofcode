use std::collections::HashMap;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)").unwrap();
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 14);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

fn part1(input: &str) -> usize {
    get_sum(input, MaskType::P1)
}

fn part2(input: &str) -> usize {
    get_sum(input, MaskType::P2)
}

fn get_sum(input: &str, mask_type: MaskType) -> usize {
    let mut v = parse(input);
    match mask_type {
        MaskType::P1 => v.iter_mut().for_each(|x| x.apply_mask()),
        MaskType::P2 => v.iter_mut().for_each(|x| x.apply_mask_v2()),
    }
    v.into_iter()
        .fold(HashMap::new(), |mut acc, x| {
            x.calculated_mem_value_pairs.into_iter().for_each(|(k, v)| {
                let _ = acc.insert(k, v);
            });
            acc
        })
        .values()
        .sum::<usize>()
}

fn parse(input: &str) -> Vec<Mask<'_>> {
    input
        .split("mask = ")
        .skip(1)
        .map(|group| {
            let mut lines = group.lines();
            let mask_str = lines.next().unwrap();
            let v = lines
                .map(|line| {
                    let capture = MEM_RE.captures(line).unwrap();
                    let mem = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let value = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    (format!("{:b}", mem), format!("{:b}", value))
                })
                .collect::<Vec<_>>();
            Mask::new(mask_str, v)
        })
        .collect::<Vec<_>>()
}

enum MaskType {
    P1,
    P2,
}

#[derive(Debug)]
struct Mask<'a> {
    mask_string: &'a str,
    mem_value_pairs: Vec<(String, String)>,
    calculated_mem_value_pairs: HashMap<usize, usize>,
}

impl<'a> Mask<'a> {
    fn new(mask_string: &'a str, mem_value_pairs: Vec<(String, String)>) -> Self {
        Self {
            mask_string,
            mem_value_pairs,
            calculated_mem_value_pairs: HashMap::new(),
        }
    }

    fn apply_mask(&mut self) {
        for (mem_index, given) in self.mem_value_pairs.iter() {
            let mut value = 0;
            for (exp, mask_c) in self.mask_string.chars().rev().enumerate() {
                match mask_c {
                    '1' => value += 2usize.pow(exp as u32),
                    '0' => (),
                    'X' => {
                        if let Some(c) = given.chars().rev().nth(exp) {
                            match c {
                                '1' => value += 2usize.pow(exp as u32),
                                '0' => (),
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            self.calculated_mem_value_pairs
                .insert(usize::from_str_radix(mem_index, 2).unwrap(), value);
        }
    }

    fn apply_mask_v2(&mut self) {
        let mut map = HashMap::new();
        for (mem_index, given) in self.mem_value_pairs.iter() {
            let mut value = vec![0];
            for (exp, mask_c) in self.mask_string.chars().rev().enumerate() {
                match mask_c {
                    '1' => value.iter_mut().for_each(|x| *x += 2usize.pow(exp as u32)),
                    '0' => {
                        if let Some(c) = mem_index.chars().rev().nth(exp) {
                            match c {
                                '1' => value.iter_mut().for_each(|x| *x += 2usize.pow(exp as u32)),
                                '0' => (),
                                _ => unreachable!(),
                            }
                        }
                    }
                    'X' => {
                        let mut clone = value.clone();
                        clone.iter_mut().for_each(|x| *x += 2usize.pow(exp as u32));
                        value.extend(clone);
                    }
                    _ => unreachable!(),
                }
            }
            value.iter().for_each(|x| {
                let _ = map.insert(*x, usize::from_str_radix(given, 2).unwrap());
            });
        }
        self.calculated_mem_value_pairs = map;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_14_test_1.in");
        let p1 = part1(&input);
        assert_eq!(165, p1);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_14_test_2.in");
        let p2 = part2(&input);
        assert_eq!(208, p2);
    }
}
