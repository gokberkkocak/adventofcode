use std::collections::HashMap;

pub fn run() {
    let input = crate::util::get_puzzle_input(2018, 13);
    let p1 = part1(&input);
    println!("p1 {},{}", p1.x, p1.y);
    let p2 = part2(&input);
    println!("p1 {},{}", p2.x, p2.y);
}

fn part1(input: &str) -> Point {
    solve(input, Strategy::AbortAtFirstCrash)
}

fn part2(input: &str) -> Point {
    solve(input, Strategy::FindLastCart)
}

fn solve(input: &str, strategy: Strategy) -> Point {
    let (roads, mut carts) = parse(&input, strategy);
    loop {
        if let Some(p) = carts.apply_step(&roads) {
            return p;
        }
    }
}

#[derive(Debug)]
struct Road {
    location: Point,
    connections: Vec<Point>,
}

impl Road {
    fn new(location: Point, connections: Vec<Point>) -> Self {
        Self {
            location,
            connections,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Cart {
    location: Point,
    current_choice: CurrentChoice,
    facing: Facing,
}

impl Cart {
    fn new(location: Point, facing: Facing) -> Self {
        Self {
            location,
            current_choice: CurrentChoice::new(),
            facing,
        }
    }
    fn take_step(&mut self, road_network: &RoadNetwork) -> Point {
        let next_point;
        match self.facing {
            Facing::Up => {
                next_point = Point::new(self.location.x as isize, self.location.y - 1 as isize);
            }
            Facing::Down => {
                next_point = Point::new(self.location.x as isize, self.location.y + 1 as isize);
            }
            Facing::Left => {
                next_point = Point::new(self.location.x as isize - 1, self.location.y as isize);
            }
            Facing::Right => {
                next_point = Point::new(self.location.x as isize + 1, self.location.y as isize);
            }
        }
        // set facing
        if let Some(road) = road_network.roads.get(&next_point) {
            if road.connections.len() == 4 {
                match self.current_choice {
                    CurrentChoice::Left => match self.facing {
                        Facing::Up => self.facing = Facing::Left,
                        Facing::Down => self.facing = Facing::Right,
                        Facing::Left => self.facing = Facing::Down,
                        Facing::Right => self.facing = Facing::Up,
                    },
                    CurrentChoice::Straight => {}
                    CurrentChoice::Right => match self.facing {
                        Facing::Up => self.facing = Facing::Right,
                        Facing::Down => self.facing = Facing::Left,
                        Facing::Left => self.facing = Facing::Up,
                        Facing::Right => self.facing = Facing::Down,
                    },
                }
                self.current_choice.next();
            } else if road.connections.len() == 2 {
                debug_assert_eq!(
                    road.connections
                        .iter()
                        .filter(|&&r| r != self.location)
                        .count(),
                    1
                );
                let next_next_point = road
                    .connections
                    .iter()
                    .filter(|&&r| r != self.location)
                    .next()
                    .unwrap();
                match (
                    next_next_point.x - next_point.x,
                    next_next_point.y - next_point.y,
                ) {
                    (1, 0) => self.facing = Facing::Right,
                    (-1, 0) => self.facing = Facing::Left,
                    (0, -1) => self.facing = Facing::Up,
                    (0, 1) => self.facing = Facing::Down,
                    (_, _) => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
        self.location = next_point;
        next_point
    }
}
#[derive(Debug)]
enum CurrentChoice {
    Left,
    Straight,
    Right,
}
#[derive(Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl CurrentChoice {
    fn new() -> Self {
        CurrentChoice::Left
    }
    fn next(&mut self) {
        match self {
            CurrentChoice::Left => *self = CurrentChoice::Straight,
            CurrentChoice::Straight => *self = CurrentChoice::Right,
            CurrentChoice::Right => *self = CurrentChoice::Left,
        }
    }
}
#[derive(Debug)]
struct RoadNetwork {
    roads: HashMap<Point, Road>,
}

struct Carts {
    carts: HashMap<Point, Cart>,
    solve_strategy: Strategy,
}

impl Carts {
    fn apply_step(&mut self, road_network: &RoadNetwork) -> Option<Point> {
        let mut cart_vec = self.carts.keys().cloned().collect::<Vec<_>>();
        let mut removed = vec![];
        cart_vec.sort_by_key(|&p| (p.y, p.x));
        for p in cart_vec {
            if removed.contains(&p) {
                continue;
            }
            let new_point = self.carts.get_mut(&p).unwrap().take_step(road_network);
            // crash detection
            if self.carts.contains_key(&new_point) {
                match self.solve_strategy {
                    Strategy::AbortAtFirstCrash => return Some(new_point),
                    Strategy::FindLastCart => {
                        let _c = self.carts.remove(&p).unwrap();
                        let _c = self.carts.remove(&new_point).unwrap();
                        removed.push(new_point);
                    }
                }
            } else {
                let c = self.carts.remove(&p).unwrap();
                self.carts.insert(new_point, c);
            }
        }
        match self.solve_strategy {
            Strategy::AbortAtFirstCrash => None,
            Strategy::FindLastCart => {
                if self.carts.len() == 1 {
                    self.carts.keys().cloned().next()
                } else {
                    None
                }
            }
        }
    }
}

enum Strategy {
    AbortAtFirstCrash,
    FindLastCart,
}

fn parse(input: &str, strategy: Strategy) -> (RoadNetwork, Carts) {
    let mut roads = HashMap::new();
    let mut carts = HashMap::new();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let current_point = Point::new(i as isize, j as isize);
            match c {
                '-' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize - 1, j as isize),
                                Point::new(i as isize + 1, j as isize),
                            ],
                        ),
                    );
                }
                '|' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize, j as isize - 1),
                                Point::new(i as isize, j as isize + 1),
                            ],
                        ),
                    );
                }
                '/' => {
                    if let Some(_n) = roads
                        .get(&Point::new(i as isize - 1, j as isize))
                        .filter(|&n| n.connections.contains(&current_point))
                    {
                        roads.insert(
                            current_point,
                            Road::new(
                                current_point,
                                vec![
                                    Point::new(i as isize - 1, j as isize),
                                    Point::new(i as isize, j as isize - 1),
                                ],
                            ),
                        );
                    } else {
                        roads.insert(
                            current_point,
                            Road::new(
                                current_point,
                                vec![
                                    Point::new(i as isize + 1, j as isize),
                                    Point::new(i as isize, j as isize + 1),
                                ],
                            ),
                        );
                    }
                }
                '\\' => {
                    if let Some(_n) = roads
                        .get(&Point::new(i as isize - 1, j as isize))
                        .filter(|&n| n.connections.contains(&current_point))
                    {
                        roads.insert(
                            current_point,
                            Road::new(
                                current_point,
                                vec![
                                    Point::new(i as isize - 1, j as isize),
                                    Point::new(i as isize, j as isize + 1),
                                ],
                            ),
                        );
                    } else {
                        roads.insert(
                            current_point,
                            Road::new(
                                current_point,
                                vec![
                                    Point::new(i as isize + 1, j as isize),
                                    Point::new(i as isize, j as isize - 1),
                                ],
                            ),
                        );
                    }
                }
                '+' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize, j as isize - 1),
                                Point::new(i as isize, j as isize + 1),
                                Point::new(i as isize + 1, j as isize),
                                Point::new(i as isize - 1, j as isize),
                            ],
                        ),
                    );
                }
                '>' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize - 1, j as isize),
                                Point::new(i as isize + 1, j as isize),
                            ],
                        ),
                    );
                    let cart = Cart::new(current_point, Facing::Right);
                    carts.insert(current_point, cart);
                }
                '<' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize - 1, j as isize),
                                Point::new(i as isize + 1, j as isize),
                            ],
                        ),
                    );
                    let cart = Cart::new(current_point, Facing::Left);
                    carts.insert(current_point, cart);
                }
                '^' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize, j as isize - 1),
                                Point::new(i as isize, j as isize + 1),
                            ],
                        ),
                    );
                    let cart = Cart::new(current_point, Facing::Up);
                    carts.insert(current_point, cart);
                }
                'v' => {
                    roads.insert(
                        current_point,
                        Road::new(
                            current_point,
                            vec![
                                Point::new(i as isize, j as isize - 1),
                                Point::new(i as isize, j as isize + 1),
                            ],
                        ),
                    );
                    let cart = Cart::new(current_point, Facing::Down);
                    carts.insert(current_point, cart);
                }
                ' ' => {}
                _ => unreachable!(),
            }
        }
    }
    (
        RoadNetwork { roads },
        Carts {
            carts,
            solve_strategy: strategy,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2018_13_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, Point::new(7, 3));
    }
}
