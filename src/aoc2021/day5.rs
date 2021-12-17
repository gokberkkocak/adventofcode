use core::fmt;

use once_cell::sync::OnceCell;
use regex::Regex;

static REG: OnceCell<Regex> = OnceCell::new();

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 5);
    let v = parse(&input);
    let p1 = part1(&v);
    println!("Part 1: {}", p1);
    let p2 = part2(&v);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    REG.get_or_init(|| regex::Regex::new(r"^(\d+),(\d+)\s+->\s+(\d+),(\d+)").unwrap());
    input
        .lines()
        .map(|line| {
            let matches = REG.get().unwrap().captures(line).unwrap();
            let x_1 = matches.get(1).unwrap().as_str().parse().unwrap();
            let y_1 = matches.get(2).unwrap().as_str().parse().unwrap();
            let x_2 = matches.get(3).unwrap().as_str().parse().unwrap();
            let y_2 = matches.get(4).unwrap().as_str().parse().unwrap();
            (Point::new(x_1, y_1), Point::new(x_2, y_2))
        })
        .collect()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    // get the points in the middle
    fn get_middle_points(&self, other: &Self) -> Vec<Point> {
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();
        let (mut x, mut y) = (self.x, self.y);
        let mut v = vec![];
        while (x, y) != (other.x + dx, other.y + dy) {
            v.push(Point::new(x, y));
            x += dx;
            y += dy;
        }
        v
    }
}

struct Graph {
    grid: Vec<Vec<i32>>,
}

impl Graph {
    fn new(points: &[(Point, Point)]) -> Self {
        let max_point = points
            .iter()
            .scan(Point::new(0, 0), |max_point, (p1, p2)| {
                max_point.x = max_point.x.max(p1.x.max(p2.x));
                max_point.y = max_point.y.max(p1.y.max(p2.y));
                Some(*max_point)
            })
            .last()
            .unwrap();
        let mut grid = vec![vec![0; max_point.x as usize + 1]; max_point.y as usize + 1];
        points.iter().for_each(|(p1, p2)| {
            let mid = p1.get_middle_points(p2);
            mid.iter().for_each(|p| {
                grid[p.y as usize][p.x as usize] += 1;
            });
        });
        Graph { grid }
    }
    // Count how many values larger than 2 or equal in the grid
    fn how_many_dangerous(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&v| *v >= 2)
            .count()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(v: &[(Point, Point)]) -> usize {
    let filtered_v = v
        .iter()
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
        .copied()
        .collect::<Vec<_>>();
    let g = Graph::new(&filtered_v);
    g.how_many_dangerous()
}

fn part2(v: &[(Point, Point)]) -> usize {
    let g = Graph::new(v);
    g.how_many_dangerous()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_5_test.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 5);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_5_test.in");
        let v = parse(&input);
        let p2 = part2(&v);
        assert_eq!(p2, 12);
    }
}
