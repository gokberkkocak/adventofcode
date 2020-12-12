use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\w)(\d+)").unwrap();
}

struct FerryState {
    x: isize,
    y: isize,
    facing: Direction,
    waypoint: WayPoint,
}

impl FerryState {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Direction::E,
            waypoint: WayPoint::new(10, 1),
        }
    }

    fn execute_basic_move(&mut self, direction: Direction, magnitude: isize) {
        match direction {
            Direction::N => self.y += magnitude,
            Direction::S => self.y -= magnitude,
            Direction::E => self.x += magnitude,
            Direction::W => self.x -= magnitude,
        }
    }

    fn execute_waypoint(&mut self, magnitude: isize) {
        self.x += magnitude * self.waypoint.x;
        self.y += magnitude * self.waypoint.y;
    }

    fn execute_all_input(&mut self, move_input: &str, move_type: MoveType) {
        move_input.lines().for_each(|l| {
            let matches = REG.captures(l).unwrap();
            let ins = matches.get(1).unwrap().as_str().chars().next().unwrap();
            let value = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();
            match move_type {
                MoveType::P1 => match ins {
                    'L' | 'R' => self.facing.turn(Turn::from(ins), value),
                    'N' | 'E' | 'S' | 'W' => self.execute_basic_move(Direction::from(ins), value),
                    'F' => self.execute_basic_move(self.facing, value),
                    _ => unreachable!(),
                },
                MoveType::P2 => match ins {
                    'L' | 'R' => self.waypoint.turn_waypoint(Turn::from(ins), value),
                    'N' | 'E' | 'S' | 'W' => {
                        self.waypoint.update_waypoint(Direction::from(ins), value)
                    }
                    'F' => self.execute_waypoint(value),
                    _ => unreachable!(),
                },
            }
        });
    }

    fn calculate_man_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

enum MoveType {
    P1,
    P2,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn from(c: char) -> Turn {
        match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => unreachable!(),
        }
    }
    fn turn_left(&mut self) {
        match self {
            Direction::N => *self = Direction::W,
            Direction::E => *self = Direction::N,
            Direction::S => *self = Direction::E,
            Direction::W => *self = Direction::S,
        }
    }
    fn turn_right(&mut self) {
        match self {
            Direction::N => *self = Direction::E,
            Direction::E => *self = Direction::S,
            Direction::S => *self = Direction::W,
            Direction::W => *self = Direction::N,
        }
    }

    fn turn(&mut self, turn: Turn, angle: isize) {
        debug_assert_eq!(angle % 90, 0);
        let times = angle / 90;
        (0..times).for_each(|_| match turn {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
        });
    }
}

struct WayPoint {
    x: isize,
    y: isize,
}

impl WayPoint {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn update_waypoint(&mut self, direction: Direction, magnitude: isize) {
        match direction {
            Direction::N => self.y += magnitude,
            Direction::S => self.y -= magnitude,
            Direction::E => self.x += magnitude,
            Direction::W => self.x -= magnitude,
        }
    }

    fn turn_waypoint(&mut self, turn: Turn, angle: isize) {
        debug_assert_eq!(angle % 90, 0);
        let times = angle / 90;
        (0..times).for_each(|_| match turn {
            Turn::Left => {
                // cos a + 90 = - sin a
                // sin a + 90 = cos a 
                std::mem::swap(&mut self.x, &mut self.y);
                self.x = -self.x;
            }
            Turn::Right => {
                // cos a - 90 = sin a
                // sin a - 90 = - cos a 
                std::mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y;
            }
        });
    }
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 12);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

fn part1(input: &str) -> usize {
    let mut ferry = FerryState::new();
    ferry.execute_all_input(input, MoveType::P1);
    ferry.calculate_man_distance()
}

fn part2(input: &str) -> usize {
    let mut ferry = FerryState::new();
    ferry.execute_all_input(input, MoveType::P2);
    ferry.calculate_man_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_12_test.in");
        let p1 = part1(&input);
        assert_eq!(25, p1);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_12_test.in");
        let p2 = part2(&input);
        assert_eq!(286, p2);
    }
}
