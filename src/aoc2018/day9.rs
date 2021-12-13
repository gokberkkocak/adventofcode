use std::collections::VecDeque;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2018, 9);
    let mut it = input.split_ascii_whitespace();
    let nb_players = it.next().unwrap().parse::<usize>().unwrap();
    let last_point = it.skip(5).next().unwrap().parse::<usize>().unwrap();
    solve(nb_players, last_point * 100);
}

fn solve(nb_players: usize, last_point: usize) {
    let mut points = vec![0; nb_players];
    let mut board = VecDeque::new();
    board.push_back(0);
    for (s, p) in (1usize..).zip((0..nb_players).cycle()).take(last_point) {
        if s % 23 == 0 {
            board.rotate_right(7);
            points[p] += s + board.pop_back().unwrap();
            board.rotate_left(1);
        } else {
            board.rotate_left(1);
            board.push_back(s);
        }
    }
    println!("{}", points.iter().max().unwrap());
}
