use std::collections::HashMap;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 10);
    let mut v = input
        .lines()
        .map(|l| l.parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    v.push(0);
    v.push(v.iter().max().unwrap() + 3);
    v.sort_unstable();
    let p1 = part1(&v);
    println!("{}", p1);
    let p2 = part2(&v);
    println!("{}", p2);
}

fn part1(v: &[u16]) -> u16 {
    let mut map = HashMap::new();
    for (&i, &j) in v.iter().zip(v.iter().skip(1)) {
        map.entry(j - i).and_modify(|v| *v += 1).or_insert(1);
    }
    map.get(&1).unwrap() * map.get(&3).unwrap()
}

fn part2(v: &[u16]) -> usize {
    let mut reach_vec = vec![0; *v.last().unwrap() as usize + 1];
    reach_vec[0] = 1;
    for &i in v.iter() {
        match i {
            _ if i == 0 => (),
            _ if i == 1 => reach_vec[i as usize] += reach_vec[i as usize - 1],
            _ if i == 2 => {
                reach_vec[i as usize] += reach_vec[i as usize - 1] + reach_vec[i as usize - 2]
            }
            _ => {
                reach_vec[i as usize] += reach_vec[i as usize - 1]
                    + reach_vec[i as usize - 2]
                    + reach_vec[i as usize - 3]
            }
        }
    }
    reach_vec[reach_vec.len() - 1]
}

#[allow(dead_code)]
fn part2_recursive(v: &[u16]) -> usize {
    let mut map = HashMap::new();
    number_of_arrangements(v, 0, &mut map)
}

#[allow(dead_code)]
fn number_of_arrangements(v: &[u16], index: usize, map: &mut HashMap<usize, usize>) -> usize {
    if index == v.len() - 1 {
        return 1;
    }
    let mut next_index = index + 1;
    let mut result = 0;
    while next_index < v.len() && v[index] + 3 >= v[next_index] {
        let count;
        match map.get(&next_index) {
            Some(value) => {
                count = *value;
            }
            None => {
                count = number_of_arrangements(v, next_index, map);
                map.insert(next_index, count);
            }
        }
        result += count;
        next_index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut v = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        v.push(0);
        v.push(v.iter().max().unwrap() + 3);
        v.sort();
        assert_eq!(35, part1(&v));
        assert_eq!(8, part2_recursive(&v));
        assert_eq!(8, part2(&v));
    }
}
