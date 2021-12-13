use std::collections::HashMap;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2018, 6);
    // let input = include_str!("../input.txt");
    let mut locs = vec![];
    for line in input.lines() {
        let mut parts = line.trim_end().split(", ");
        let x = parts.next().unwrap().parse::<u32>().unwrap();
        let y = parts.next().unwrap().parse::<u32>().unwrap();
        locs.push((x, y));
    }
    let x_min = locs.iter().min_by_key(|&(x, _)| x).unwrap().0;
    let x_max = locs.iter().max_by_key(|&(x, _)| x).unwrap().0;
    let y_min = locs.iter().min_by_key(|&(_, y)| y).unwrap().1;
    let y_max = locs.iter().max_by_key(|&(_, y)| y).unwrap().1;
    println!("{} {}", x_min, y_min);
    println!("{} {}", x_max, y_max);
    let x_len = x_max - x_min + 1;
    let y_len = y_max - y_min + 1;
    for (i, j) in &mut locs {
        *i = *i - x_min;
        *j = *j - y_min;
    }

    let mut board = vec![vec![".".to_string(); y_len as usize]; x_len as usize];
    part1(&mut board, &mut locs);
    part2(&board, &locs);
}

pub fn calculate_man_distance(p1: (u32, u32), p2: (u32, u32)) -> u64 {
    ((p1.0 as i64 - p2.0 as i64).abs() + (p1.1 as i64 - p2.1 as i64).abs()) as u64
}

pub fn part1(board: &mut Vec<Vec<String>>, locs: &Vec<(u32, u32)>) {
    for (i, v) in board.iter_mut().enumerate() {
        for (j, s) in v.iter_mut().enumerate() {
            let mut min_distance = u64::MAX;
            let mut winner: Option<usize> = None;
            for (index, l) in locs.iter().enumerate() {
                let d = calculate_man_distance((i as u32, j as u32), *l);
                if d < min_distance {
                    winner = Some(index);
                    min_distance = d;
                } else if d == min_distance {
                    winner = None;
                }
            }
            if let Some(winner) = winner {
                s.clear();
                s.insert_str(0, &winner.to_string());
            }
        }
    }

    let mut res: HashMap<Option<usize>, Option<u64>> = HashMap::new();
    for i in 0..locs.len() {
        res.insert(Some(i), Some(0));
    }
    res.insert(None, None);

    for (i, v) in board.iter().enumerate() {
        for (j, s) in v.iter().enumerate() {
            if i == 0 || j == 0 || i == board.len() - 1 || j == v.len() - 1 {
                res.insert(s.parse::<usize>().ok(), None);
            } else {
                res.entry(s.parse::<usize>().ok()).and_modify(|v| {
                    if let Some(value) = *v {
                        *v = Some(value + 1)
                    }
                });
            }
        }
    }

    let max = res
        .iter()
        .max_by_key(|&(_, v)| if let Some(v) = v { *v } else { 0 })
        .unwrap();
    println!("p1 {}", max.1.unwrap());
}

pub fn part2(board: &Vec<Vec<String>>, locs: &Vec<(u32, u32)>) {
    let max_allowed = 10000;
    let mut count = 0;
    for (i, v) in board.iter().enumerate() {
        for (j, _) in v.iter().enumerate() {
            let mut total = 0;
            for (_, l) in locs.iter().enumerate() {
                let d = calculate_man_distance((i as u32, j as u32), *l);
                total += d;
            }
            if total < max_allowed {
                count += 1;
            }
        }
    }
    println!("p2 {}", count);
}
