use std::collections::BinaryHeap;

use fxhash::FxHashMap;
use fxhash::FxHashSet;

static NEIGHBOUR_INDEX: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 15);
    let mat = parse(&input);
    let p1 = part1(&mat);
    println!("Part 1: {}", p1);
    let p2 = part2(&mat);
    println!("Part 2: {}", p2);
}

fn part1(mat: &[Vec<usize>]) -> usize {
    let start = (0, 0);
    let finish = (mat.len() - 1, mat.len() - 1);
    a_star(mat, start, finish, false)
}

fn part2(mat: &[Vec<usize>]) -> usize {
    let start = (0, 0);
    let finish = (mat.len() * 5 - 1, mat.len() * 5 - 1);
    a_star(mat, start, finish, true)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn manhattan_distance(pa: (usize, usize), pb: (usize, usize)) -> usize {
    ((pb.0 as isize - pa.0 as isize).abs() + (pb.1 as isize - pa.1 as isize).abs()) as usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
    cost: usize,
    heuristic: usize,
    total: usize,
}

impl Point {
    fn new(x: usize, y: usize, cost: usize, heuristic: usize) -> Point {
        Point {
            x,
            y,
            cost,
            heuristic,
            total: cost + heuristic,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order
        other.total.cmp(&self.total)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(
    mat: &[Vec<usize>],
    start: (usize, usize),
    finish: (usize, usize),
    expand_mat: bool,
) -> usize {
    let mut opened = BinaryHeap::new();
    let mut closed = FxHashSet::default();
    let mut cheapest_cost_map = FxHashMap::default();
    cheapest_cost_map.insert(start, 0);
    let starting_point = Point::new(start.0, start.1, 0, manhattan_distance(start, finish));
    opened.push(starting_point);
    while let Some(current) = opened.pop() {
        if current.x == finish.0 && current.y == finish.1 {
            return current.cost;
        }
        closed.insert((current.x, current.y));
        for n in get_neighbour_indexes(mat, current.x, current.y, expand_mat)
            .filter(|n| !closed.contains(&(n.0, n.1)))
        {
            let new_cost = current.cost + get_point_cost(mat, n.0, n.1);
            // check it is worse than cheapest path
            if let Some(cost) = cheapest_cost_map.get(&n) {
                if new_cost >= *cost {
                    continue;
                }
            }
            cheapest_cost_map.insert(n, new_cost);
            let new_p = Point::new(n.0, n.1, new_cost, manhattan_distance(n, finish));
            opened.push(new_p);
        }
    }
    unreachable!()
}

fn get_point_cost(mat: &[Vec<usize>], x: usize, y: usize) -> usize {
    let x_level = x / mat.len();
    let y_level = y / mat.len();
    let cost = mat[y % mat.len()][x % mat.len()] + x_level + y_level;
    if cost < 10 {
        cost
    } else {
        cost - 9
    }
}

fn get_neighbour_indexes(
    mat: &[Vec<usize>],
    x: usize,
    y: usize,
    expand_mat: bool,
) -> impl Iterator<Item = (usize, usize)> {
    let mut len = mat.len() as isize;
    // expand the mat if needed
    if expand_mat {
        len *= 5;
    }
    NEIGHBOUR_INDEX
        .iter()
        .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(move |(x, y)| *x >= 0 && *y >= 0 && *x < len && *y < len)
        .map(|(x, y)| (x as usize, y as usize))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_15_test.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 40);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_15_test.in");
        let v = parse(&input);
        let p1 = part2(&v);
        assert_eq!(p1, 315);
    }
}
