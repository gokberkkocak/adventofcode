use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"x|\d+").unwrap();
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 13);
    let (earliest, mut buses) = parse(&input);
    let p1 = part1(&buses, earliest);
    println!("p1 {}", p1);
    let p2 = part2(&mut buses);
    println!("p2 {}", p2);
}

fn parse(input: &str) -> (usize, Vec<BusWait>) {
    let mut it = input.lines();
    let earliest = it.next().unwrap().parse::<usize>().unwrap();
    let mut bus_count = 0;
    let buses = REG
        .captures_iter(it.next().unwrap())
        .fold(vec![], |mut acc, bus| {
            if bus.get(0).unwrap().as_str() != "x" {
                acc.push(BusWait::new(
                    bus.get(0).unwrap().as_str().parse::<usize>().unwrap(),
                    bus_count,
                ));
            }
            bus_count += 1;
            acc
        });
    (earliest, buses)
}

fn part1(buses: &[BusWait], earliest: usize) -> usize {
    let (b, m) = buses
        .iter()
        .map(|b| (b.bus_id, b.get_next_departure(earliest)))
        .min_by_key(|&(_, m)| m)
        .unwrap();
    b * m
}

fn part2(buses: &mut Vec<BusWait>) -> usize {
    let mut increment = 1;
    let mut count = 0;
    while !buses.is_empty() {
        count += increment;
        while let Some((i, bus)) = buses.iter().enumerate().find(|(_, b)| b.is_match(count)) {
            increment *= bus.bus_id;
            buses.remove(i);
        }
    }
    count
}
#[derive(Debug)]
struct BusWait {
    bus_id: usize,
    req_wait: usize,
}

impl BusWait {
    fn new(bus_id: usize, req_wait: usize) -> Self {
        Self {
            bus_id,
            req_wait: req_wait % bus_id,
        }
    }
    fn get_next_departure(&self, time: usize) -> usize {
        (self.bus_id - time % self.bus_id) % self.bus_id
    }

    fn is_match(&self, time: usize) -> bool {
        self.get_next_departure(time) == self.req_wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_13_test.in");
        let (e, b) = parse(&input);
        let p1 = part1(&b, e);
        assert_eq!(295, p1);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_13_test.in");
        let (_e, mut b) = parse(&input);
        let p2 = part2(&mut b);
        assert_eq!(1068781, p2);
    }
}
