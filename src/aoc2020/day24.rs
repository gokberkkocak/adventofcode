use std::collections::HashSet;

use lazy_static::lazy_static;

use regex::Regex;

static NEIGHBOUR_INDEX: [(i8, i8); 6] = [(-1, -1), (-1, 1), (1, -1), (1, 1), (2, 0), (-2, 0)];

lazy_static! {
    static ref RE: Regex = Regex::new("[ns]?[ew]").unwrap();
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 24);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

struct Tiles {
    black: HashSet<(isize, isize)>,
    x_max: isize,
    y_max: isize,
    x_min: isize,
    y_min: isize,
}

impl Tiles {
    fn new(input_vec: Vec<(isize, isize)>) -> Self {
        let mut black = HashSet::new();
        for i in input_vec {
            if black.contains(&i) {
                black.remove(&i);
            } else {
                black.insert(i);
            }
        }
        let mut s = Self {
            black,
            x_max: 0,
            y_max: 0,
            x_min: 0,
            y_min: 0,
        };
        s.adjust_boundaries();
        s
    }

    fn adjust_boundaries(&mut self) {
        self.x_max = self.black.iter().map(|(x, _y)| x).max().unwrap() + 2; // east and west are 2 apart in 2d plane
        self.x_min = self.black.iter().map(|(x, _y)| x).min().unwrap() - 2;
        self.y_max = self.black.iter().map(|(_x, y)| y).max().unwrap() + 1;
        self.y_min = self.black.iter().map(|(_x, y)| y).min().unwrap() - 1;
    }

    fn daily_flip(&mut self) {
        let mut new_day_black = HashSet::new();
        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                if self.black.contains(&(x, y)) {
                    let black_n_count = NEIGHBOUR_INDEX
                        .iter()
                        .filter(|(i, j)| self.black.contains(&(x + *i as isize, y + *j as isize)))
                        .count();
                    if !(black_n_count == 0 || black_n_count > 2) {
                        new_day_black.insert((x, y));
                    }
                } else {
                    let black_n_count = NEIGHBOUR_INDEX
                        .iter()
                        .filter(|(i, j)| self.black.contains(&(x + *i as isize, y + *j as isize)))
                        .count();
                    if black_n_count == 2 {
                        new_day_black.insert((x, y));
                    }
                }
            }
        }
        self.black = new_day_black;
        self.adjust_boundaries()
    }

    fn flip_by_day(&mut self, day: usize) {
        for _ in 0..day {
            self.daily_flip();
        }
    }
}

/*
*       nw      ne
*   w       ()      e
*       sw      se
*/

fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| {
            RE.captures_iter(line)
                .map(|c| match c.get(0).unwrap().as_str() {
                    "ne" => (1, 1),
                    "se" => (1, -1),
                    "nw" => (-1, 1),
                    "sw" => (-1, -1),
                    "e" => (2, 0),
                    "w" => (-2, 0),
                    _ => unreachable!(),
                })
                .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let v = parse(input);
    let tiles = Tiles::new(v);
    tiles.black.len()
}

fn part2(input: &str) -> usize {
    let v = parse(input);
    let mut tiles = Tiles::new(v);
    tiles.flip_by_day(100);
    tiles.black.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 10);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let v = parse(&input);
        let mut tiles = Tiles::new(v);
        tiles.flip_by_day(1);
        assert_eq!(tiles.black.len(), 15);
    }

    #[test]
    fn test_3() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let v = parse(&input);
        let mut tiles = Tiles::new(v);
        tiles.flip_by_day(2);
        assert_eq!(tiles.black.len(), 12);
    }

    #[test]
    fn test_4() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let v = parse(&input);
        let mut tiles = Tiles::new(v);
        tiles.flip_by_day(3);
        assert_eq!(tiles.black.len(), 25);
    }
    #[test]
    fn test_5() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let v = parse(&input);
        let mut tiles = Tiles::new(v);
        tiles.flip_by_day(90);
        assert_eq!(tiles.black.len(), 1844);
    }
    #[test]
    fn test_6() {
        let input = crate::util::read_file("inputs/2020_24_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 2208);
    }
}
