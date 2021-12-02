use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\w+)\s(\d+)").unwrap();
}

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 2);
    let v = parse(&input);
    let p1 = part1(&v);
    println!("p1: {}", p1);
    let p2 = part2(&v);
    println!("p2: {}", p2);
}

enum Op {
    Forward,
    Up,
    Down,
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        match input {
            "forward" => Op::Forward,
            "up" => Op::Up,
            "down" => Op::Down,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<(Op, usize)> {
    input
        .lines()
        .map(|l| {
            let matches = REG.captures(l).unwrap();
            let op = Op::from(matches.get(1).unwrap().as_str());
            let value = matches.get(2).unwrap().as_str().parse().unwrap();
            (op, value)
        })
        .collect()
}

fn part1(v: &[(Op, usize)]) -> usize {
    let (h, d) = v.iter().fold((0, 0), |(mut h, mut d), x| {
        match x.0 {
            Op::Forward => h += x.1,
            Op::Up => d -= x.1,
            Op::Down => d += x.1,
        }
        (h, d)
    });
    h * d
}

fn part2(v: &[(Op, usize)]) -> usize {
    let (h, d, _) = v.iter().fold((0, 0, 0), |(mut h, mut d, mut aim), x| {
        match x.0 {
            Op::Forward => {
                h += x.1;
                d += aim * x.1;
            }
            Op::Up => aim -= x.1,
            Op::Down => aim += x.1,
        }
        (h, d, aim)
    });
    h * d
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_2_test.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 150);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_2_test.in");
        let v = parse(&input);
        let p2 = part2(&v);
        assert_eq!(p2, 900);
    }
}
