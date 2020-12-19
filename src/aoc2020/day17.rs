use std::collections::HashSet;

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 17);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

fn part1(input: &str) -> usize {
    let mut pd = parse(&input, SolveType::P1);
    pd.solve();
    pd.get_nb_active()
}

fn part2(input: &str) -> usize {
    let mut pd = parse(&input, SolveType::P2);
    pd.solve();
    pd.get_nb_active()
}

fn parse(input: &str, solve_type: SolveType) -> PocketDimension {
    let mut points = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    points.insert(Point::new(i as isize, j as isize, 0, 0));
                }
                _ => {}
            }
            // points.insert(Point::new(i as isize, j as isize, 0, 0), is_active);
        }
    }
    let pd = PocketDimension::new(points, solve_type);
    pd
}

enum SolveType {
    P1,
    P2,
}

struct PocketDimension {
    points: HashSet<Point>,
    min_bound: (isize, isize, isize, isize),
    max_bound: (isize, isize, isize, isize),
    solve_type: SolveType,
}

impl PocketDimension {
    fn new(points: HashSet<Point>, solve_type: SolveType) -> Self {
        let mut s = Self {
            points,
            min_bound: (0, 0, 0, 0),
            max_bound: (0, 0, 0, 0),
            solve_type,
        };
        s.adjust_boundaries();
        s
    }

    fn adjust_boundaries(&mut self) {
        let min_x = self.points.iter().min_by_key(|&p| p.x).unwrap().x - 1;
        let max_x = self.points.iter().max_by_key(|&p| p.x).unwrap().x + 1;
        let min_y = self.points.iter().min_by_key(|&p| p.y).unwrap().y - 1;
        let max_y = self.points.iter().max_by_key(|&p| p.y).unwrap().y + 1;
        let min_z = self.points.iter().min_by_key(|&p| p.z).unwrap().z - 1;
        let max_z = self.points.iter().max_by_key(|&p| p.z).unwrap().z + 1;
        let min_w;
        let max_w;
        match &self.solve_type {
            SolveType::P1 => {
                min_w = 0;
                max_w = 0;
            }
            SolveType::P2 => {
                min_w = self.points.iter().min_by_key(|&p| p.w).unwrap().w - 1;
                max_w = self.points.iter().max_by_key(|&p| p.w).unwrap().w + 1;
            }
        }

        self.min_bound = (min_x, min_y, min_z, min_w);
        self.max_bound = (max_x, max_y, max_z, max_w);
    }

    fn apply_cycle(&mut self) {
        let mut next_cycle_points = HashSet::new();
        for i in self.min_bound.0..=self.max_bound.0 {
            for j in self.min_bound.1..=self.max_bound.1 {
                for k in self.min_bound.2..=self.max_bound.2 {
                    for l in self.min_bound.3..=self.max_bound.3 {
                        let p = Point::new(i, j, k, l);
                        let neighbours;
                        match &self.solve_type {
                            SolveType::P1 => neighbours = p.get_neighbours_3d(),
                            SolveType::P2 => neighbours = p.get_neighbours_4d(),
                        }
                        let active_n_count = neighbours
                            .iter()
                            .map(|&p2| self.points.contains(&p2))
                            .filter(|&b| b)
                            .count();
                        match self.points.contains(&p) {
                            true => {
                                if !(active_n_count > 3 || active_n_count < 2) {
                                    next_cycle_points.insert(p);
                                }
                            }
                            false => {
                                if active_n_count == 3 {
                                    next_cycle_points.insert(p);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.points = next_cycle_points;
        self.adjust_boundaries()
    }

    fn get_nb_active(&self) -> usize {
        self.points.iter().count()
    }

    fn solve(&mut self) {
        for _ in 0..6 {
            self.apply_cycle();
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }

    fn get_neighbours_3d(&self) -> Vec<Point> {
        let mut v = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if !(i == 0 && j == 0 && k == 0) {
                        v.push(Point::new(self.x + i, self.y + j, self.z + k, 0));
                    }
                }
            }
        }
        v
    }

    fn get_neighbours_4d(&self) -> Vec<Point> {
        let mut v = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if !(i == 0 && j == 0 && k == 0 && l == 0) {
                            v.push(Point::new(self.x + i, self.y + j, self.z + k, self.w + l));
                        }
                    }
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_17_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 112);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_17_test.in");
        let p1 = part2(&input);
        assert_eq!(p1, 848);
    }
}
