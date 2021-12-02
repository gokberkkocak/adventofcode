use std::collections::HashMap;

use crate::util::get_puzzle_input;

static NEIGHBOUR_INDEX: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[derive(Debug, Clone)]
struct Area {
    seats: HashMap<Point, Seat>,
    _x_len: isize,
    _y_len: isize,
    round: usize,
}

impl Area {
    fn new(input: &str) -> Self {
        let mut seats = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                seats.insert(Point(i as isize, j as isize), Seat::convert_from_char(c));
            }
        }
        let x_len = seats.keys().max_by_key(|&p| p.0).unwrap().0 + 1;
        let y_len = seats.keys().max_by_key(|&p| p.1).unwrap().1 + 1;
        Self {
            seats,
            _x_len: x_len,
            _y_len: y_len,
            round: 0,
        }
    }

    fn get_neighbours(&self, p: &Point) -> Vec<Seat> {
        let mut v = vec![];
        NEIGHBOUR_INDEX.iter().for_each(|&(i, j)| {
            if let Some(s) = self.seats.get(&Point(p.0 + i as isize, p.1 + j as isize)) {
                v.push(*s);
            }
        });
        v
    }

    fn get_first_seat_each_direction(&self, p: &Point) -> Vec<Seat> {
        let mut v = vec![];
        NEIGHBOUR_INDEX.iter().for_each(|&(i, j)| {
            let mut x = p.0 + i as isize;
            let mut y = p.1 + j as isize;
            while let Some(p) = self.seats.get(&Point(x, y)) {
                match p {
                    Seat::EMPTY | Seat::OCCUPIED => {
                        v.push(*p);
                        break;
                    }
                    _ => {
                        x += i as isize;
                        y += j as isize;
                    }
                }
            }
        });

        v
    }

    fn get_nb_occupied_neighbours(&self, p: &Point, solve_type: SolveType) -> usize {
        let v;
        match solve_type {
            SolveType::P1 => v = self.get_neighbours(p),
            SolveType::P2 => v = self.get_first_seat_each_direction(p),
        }
        v.into_iter().filter(|&s| s == Seat::OCCUPIED).count()
    }

    fn next_round(&mut self, kind: SolveInformation) -> bool {
        self.round += 1;
        let mut changed_flag = false;
        let mut clone_seats = self.seats.clone();
        for (&p, &seat) in self.seats.iter() {
            match seat {
                Seat::EMPTY if self.get_nb_occupied_neighbours(&p, kind.solve_type) == 0 => {
                    clone_seats.entry(p).and_modify(|x| *x = Seat::OCCUPIED);
                    changed_flag = true;
                }
                Seat::OCCUPIED
                    if self.get_nb_occupied_neighbours(&p, kind.solve_type)
                        >= kind.occupancy_limit =>
                {
                    clone_seats.entry(p).and_modify(|x| *x = Seat::EMPTY);
                    changed_flag = true;
                }
                _ => (),
            }
        }
        if changed_flag {
            self.seats = clone_seats;
        }
        changed_flag
    }

    fn do_all_rounds(&mut self, kind: SolveInformation) {
        let mut changed = true;
        while changed {
            changed = self.next_round(kind);
            #[cfg(debug_assertions)]
            self.print_area();
        }
    }

    fn get_total_nb_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|&s| *s == Seat::OCCUPIED)
            .count()
    }
    #[cfg(debug_assertions)]
    fn print_area(&self) {
        println!("___");
        for i in 0..self._x_len {
            for j in 0..self._y_len {
                print!("{}", self.seats.get(&Point(i, j)).unwrap());
            }
            println!();
        }
        println!("___");
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    FLOOR,
    EMPTY,
    OCCUPIED,
}
#[derive(Debug, Copy, Clone)]
struct SolveInformation {
    occupancy_limit: usize,
    solve_type: SolveType,
}
#[derive(Debug, Copy, Clone)]
enum SolveType {
    P1,
    P2,
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FLOOR => write!(f, "."),
            Self::EMPTY => write!(f, "L"),
            Self::OCCUPIED => write!(f, "#"),
        }
    }
}

impl Seat {
    fn convert_from_char(c: char) -> Self {
        match c {
            '.' => Self::FLOOR,
            'L' => Self::EMPTY,
            '#' => Self::OCCUPIED,
            _ => unreachable!(),
        }
    }
}

pub fn run() {
    let input = get_puzzle_input(2020, 11);
    let mut area = Area::new(&input);
    let p1 = part1(&mut area);
    println!("p1 {} after {} rounds", p1, area.round);
    let mut area = Area::new(&input);
    let p2 = part2(&mut area);
    println!("p2 {} after {} rounds", p2, area.round);
}

fn part1(area: &mut Area) -> usize {
    let solve_info = SolveInformation {
        occupancy_limit: 4,
        solve_type: SolveType::P1,
    };
    area.do_all_rounds(solve_info);
    area.get_total_nb_occupied()
}

fn part2(area: &mut Area) -> usize {
    let solve_info = SolveInformation {
        occupancy_limit: 5,
        solve_type: SolveType::P2,
    };
    area.do_all_rounds(solve_info);
    area.get_total_nb_occupied()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_p1() {
        let input = crate::util::read_file("inputs/2020_11_test.in");
        let mut area = Area::new(&input);
        let p1 = part1(&mut area);
        assert_eq!(37, p1);
    }

    #[test]
    fn test_p2() {
        let input = crate::util::read_file("inputs/2020_11_test.in");
        let mut area = Area::new(&input);
        let p1 = part2(&mut area);
        assert_eq!(26, p1);
    }
}
