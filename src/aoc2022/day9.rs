use std::collections::HashSet;

pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 9);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    println!("p2: {}", p2);
}

fn parse(input: &str) -> Vec<(Direction, u8)> {
    input
        .lines()
        .map(|s| {
            let mut it = s.split_ascii_whitespace();
            let dir_str = it.next().unwrap();
            let dir = Direction::new(dir_str.chars().next().unwrap());
            let dist = it.next().unwrap().parse::<u8>().unwrap();
            (dir, dist)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let moves = parse(input);
    solve::<2>(moves)
}

fn part2(input: &str) -> usize {
    let moves = parse(input);
    solve::<10>(moves)
}

fn solve<const N: usize>(moves: Vec<(Direction, u8)>) -> usize {
    let mut points = [Point::new(0, 0); N];
    moves
        .iter()
        .flat_map(|&(direction, amount)| std::iter::repeat(direction).take(amount as usize))
        .map(|direction| {
            points[0].move_head(direction);
            (0..N - 1).for_each(|i| points[i + 1].follow(points[i]));
            points[N - 1]
        })
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    R,
    U,
    L,
    D,
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            'R' => Self::R,
            'U' => Self::U,
            'L' => Self::L,
            'D' => Self::D,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn move_head(&mut self, direction: Direction) {
        let diff = match direction {
            Direction::R => (1, 0),
            Direction::U => (0, 1),
            Direction::L => (-1, 0),
            Direction::D => (0, -1),
        };
        self.x += diff.0;
        self.y += diff.1;
    }
    fn follow(&mut self, head: Point) {
        let diff = (head.x - self.x, head.y - self.y);
        let touching = diff.0.abs() < 2 && diff.1.abs() < 2;
        if !touching {
            self.x += diff.0.signum();
            self.y += diff.1.signum();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_9_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 13);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_9_test_2.in");
        let p2 = part2(&input);
        assert_eq!(p2, 36);
    }
}
