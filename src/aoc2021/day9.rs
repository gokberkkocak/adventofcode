use std::collections::{HashMap, HashSet};

static NEIGHBOUR_INDEX: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
static MAX_HEIGHT_VALUE: u8 = 9;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 9);
    let caves = parse(&input);
    let p1 = part1(&caves);
    println!("Part 1: {}", p1);
    let p2 = part2(&caves);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> Caves {
    let area = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().map(move |(x, c)| {
                (
                    Point::new(x as isize, y as isize),
                    c.to_digit(10).unwrap() as u8,
                )
            })
        })
        .collect();
    Caves { area }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> impl Iterator<Item = Point> + '_ {
        NEIGHBOUR_INDEX
            .iter()
            .map(move |(dx, dy)| Point::new(self.x + dx, self.y + dy))
    }
}

struct Caves {
    area: HashMap<Point, u8>,
}

impl Caves {
    fn get_bottom_points(&self) -> impl Iterator<Item = (&Point, &u8)> + '_ {
        self.area.iter().filter(move |(p, v)| {
            p.neighbours()
                .all(|p2| self.area.get(&p2).unwrap_or(&(MAX_HEIGHT_VALUE + 1)) > v)
        })
    }

    fn get_basin_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.get_bottom_points().map(move |(p, _v)| {
            let mut stack = vec![*p];
            let mut result_set = HashSet::new();
            while let Some(current) = stack.pop() {
                if result_set.contains(&current) {
                    continue;
                }
                stack.extend(current.neighbours().filter(|p2| {
                    self.area.get(&p2).unwrap_or(&(MAX_HEIGHT_VALUE + 1)) < &MAX_HEIGHT_VALUE
                }));
                result_set.insert(current);
            }
            result_set.len()
        })
    }
}

fn part1(caves: &Caves) -> usize {
    caves
        .get_bottom_points()
        .map(|(_, v)| (v + 1) as usize)
        .sum()
}

fn part2(caves: &Caves) -> usize {
    let mut basin_sizes = caves.get_basin_sizes().collect::<Vec<_>>();
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_9_test.in");
        let caves = parse(&input);
        let p1 = part1(&caves);
        assert_eq!(p1, 15);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_9_test.in");
        let caves = parse(&input);
        let p2 = part2(&caves);
        assert_eq!(p2, 1134);
    }
}
