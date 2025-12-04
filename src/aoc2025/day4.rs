const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 4);
    let mut grid = Grid::parse(&input);
    let p1 = part1(&grid);
    println!("part 1: {}", p1);
    let p2 = part2(&mut grid);
    println!("part 2: {}", p2);
}

fn part1(grid: &Grid) -> usize {
    grid.get_accessible().len()
}

fn part2(grid: &mut Grid) -> usize {
    // turn accessible cells to false until no more accessible cells
    let mut count = 0;
    loop {
        let accessible_cells = grid.get_accessible();
        if accessible_cells.is_empty() {
            break;
        }
        for (x, y) in accessible_cells {
            grid.cells[y][x] = false;
            count += 1;
        }
    }
    count
}

struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        Grid {
            cells: input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|b| match b {
                            b'@' => true,
                            b'.' => false,
                            _ => panic!("invalid char"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    // true cells with fewer than 4 true neighbors (all 8) are accessible
    fn get_accessible(&self) -> Vec<(usize, usize)> {
        let mut accessible_cells = Vec::new();
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                if self.cells[y][x]
                    && NEIGHBOURS
                        .into_iter()
                        .filter(|&(dx, dy)| {
                            self.cells
                                .get(y.wrapping_add_signed(dy))
                                .and_then(|row| row.get(x.wrapping_add_signed(dx)))
                                .filter(|&c| *c)
                                .is_some()
                        })
                        .count()
                        < 4
                {
                    accessible_cells.push((x, y));
                }
            }
        }
        accessible_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_4_test.in");
        let grid = Grid::parse(&input);
        let p1 = part1(&grid);
        assert_eq!(p1, 13);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_4_test.in");
        let mut grid = Grid::parse(&input);
        let p1 = part2(&mut grid);
        assert_eq!(p1, 43);
    }
}
