use std::fmt;

use crate::util::{get_puzzle_input};

use regex::Regex;

struct Point {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Point {
    fn new(x: i32, y: i32, vel_x: i32, vel_y: i32) -> Self {
        Point { x, y, vel_x, vel_y }
    }
    fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "p({}, {}) v({} {})",
            self.x, self.y, self.vel_x, self.vel_y
        )
    }
}

pub fn run() {
    let input = get_puzzle_input(2018, 10);
    let re = Regex::new(r"position=<( *-?\d+), ( *-?\d+)> velocity=<( *-?\d), ( *-?\d)>").unwrap();
    let mut v = vec![];
    for line in input.lines() {
        let c = re.captures(line.trim()).unwrap();
        let p = Point::new(
            c[1].trim().parse::<i32>().unwrap(),
            c[2].trim().parse::<i32>().unwrap(),
            c[3].trim().parse::<i32>().unwrap(),
            c[4].trim().parse::<i32>().unwrap(),
        );
        v.push(p);
    }
    for it in 0..10942 {
        v.iter_mut().for_each(|p| p.update());
        let x_min = v.iter().min_by_key(|&p| p.x ).unwrap();
        let y_min = v.iter().min_by_key(|&p| p.y ).unwrap();
        let x_max = v.iter().max_by_key(|&p| p.x ).unwrap();
        let y_max = v.iter().max_by_key(|&p| p.y ).unwrap();
        println!("{} {} {} {} {}",it, x_max.x, x_min.x, y_max.y, y_min.y);
    }
    let mut board = vec![vec![" "; 62]; 15];
    v.iter().for_each(|p| board[p.y as usize - 140][p.x as usize - 167] = "X" );
    for i in 0..15 {
        for j in 0..62 {
            print!("{}", board[i][j]);
        }
        println!();
    }

}
