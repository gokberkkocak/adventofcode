use once_cell::sync::OnceCell;
use regex::Regex;

static REG: OnceCell<Regex> = OnceCell::new();

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 2);
    let v = parse(&input);
    let p1 = part1(&v);
    println!("p1: {}", p1);
    let p2 = part2(&v);
    println!("p2: {}", p2);
}

enum Op {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl From<(&str, usize)> for Op {
    fn from(input: (&str, usize)) -> Self {
        match input.0 {
            "forward" => Op::Forward(input.1),
            "up" => Op::Up(input.1),
            "down" => Op::Down(input.1),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Op> {
    REG.get_or_init(|| regex::Regex::new(r"^(\w+)\s(\d+)").unwrap());
    input
        .lines()
        .map(|l| {
            let matches = REG.get().unwrap().captures(l).unwrap();
            let text = matches.get(1).unwrap().as_str();
            let value = matches.get(2).unwrap().as_str().parse().unwrap();
            Op::from((text, value))
        })
        .collect()
}

fn part1(v: &[Op]) -> usize {
    let (h, d) = v.iter().fold((0, 0), |(h, d), op| match op {
        Op::Forward(i) => (h + i, d),
        Op::Up(i) => (h, d - i),
        Op::Down(i) => (h, d + i),
    });
    h * d
}

fn part2(v: &[Op]) -> usize {
    let (h, d, _) = v.iter().fold((0, 0, 0), |(h, d, aim), op| match op {
        Op::Forward(i) => (h + i, d + aim * i, aim),
        Op::Up(i) => (h, d, aim - i),
        Op::Down(i) => (h, d, aim + i),
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
