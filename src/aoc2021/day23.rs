use std::cmp::Ordering;
use std::collections::BinaryHeap;

use fxhash::FxHashSet;

static VALID_COORDS: &[(i32, i32)] = &[
    (0, 0),
    (1, 0),
    (3, 0),
    (5, 0),
    (7, 0),
    (9, 0),
    (10, 0),
    (2, 1),
    (2, 2),
    (2, 3),
    (2, 4),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4),
    (6, 1),
    (6, 2),
    (6, 3),
    (6, 4),
    (8, 1),
    (8, 2),
    (8, 3),
    (8, 4),
];

fn dist(from: u8, to: u8) -> u32 {
    let f = &VALID_COORDS[from as usize];
    let t = &VALID_COORDS[to as usize];
    (f.0 - t.0).abs() as u32
        + if f.0 == t.0 {
            (f.1 - t.1).abs() as u32
        } else {
            (f.1 + t.1) as u32
        }
}

fn try_move_down(p: &[u8; 23], it: u8) -> Option<u8> {
    debug_assert!((1..=4).contains(&it));
    let ppos = usize::from(it * 4 + 3);
    let pocket = &p[ppos..ppos + 4];
    for q in (0..=3).rev() {
        if pocket[q] == 0 {
            return Some((ppos + q) as u8);
        }
        if pocket[q] != it {
            return None;
        }
    }
    None
}

fn try_move_up(p: &[u8; 23], it: u8) -> Option<u8> {
    let ppos = usize::from(it * 4 + 3);
    let pocket = &p[ppos..ppos + 4];
    for q in 0..=3 {
        if pocket[q] != 0 {
            if pocket[q..].iter().all(|x| *x == it) {
                return None;
            }
            return Some((ppos + q) as u8);
        }
    }
    None
}

fn possible_moves(p: &[u8; 23]) -> Vec<(u8, u8)> {
    let mut results = Vec::<(u8, u8)>::new();
    'up: for f in 1..=4 {
        if let Some(up) = try_move_up(p, f) {
            // left
            for left_pos in (0..=f).rev() {
                if p[left_pos as usize] == 0 {
                    results.push((up, left_pos));
                } else {
                    break;
                }
            }
            // right
            for right_pos in f + 1..=6 {
                if p[right_pos as usize] == 0 {
                    results.push((up, right_pos));
                } else {
                    break;
                }
            }
            // direct down, verify intermediate positions
            if p[up as usize] == f {
                continue;
            }
            if let Some(down) = try_move_down(p, p[up as usize]) {
                let right_target = p[up as usize];
                let left_target = p[up as usize] + 1;
                if f < right_target {
                    for i in f + 1..=right_target {
                        if p[i as usize] != 0 {
                            continue 'up;
                        }
                    }
                } else {
                    for i in left_target..=f {
                        if p[i as usize] != 0 {
                            continue 'up;
                        }
                    }
                }
                results.push((up as u8, down))
            }
        }
    }
    'down: for from in 0..=6 {
        if p[from] == 0 {
            continue;
        }
        if let Some(down) = try_move_down(p, p[from]) {
            let right_target = p[from] as usize;
            let left_target = p[from] as usize + 1;
            if from <= right_target {
                if p[from + 1..=right_target].iter().any(|x| *x != 0) {
                    continue 'down;
                }
            } else if p[left_target..from].iter().any(|x| *x != 0) {
                continue 'down;
            }
            results.push((from as u8, down))
        }
    }
    results
}

fn do_move(p: &[u8; 23], m: (u8, u8)) -> ([u8; 23], u32) {
    let d = dist(m.0, m.1);
    let step: u32 = match p[m.0 as usize] {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => unreachable!(),
    };
    let mut new_p = *p;
    new_p[m.1 as usize] = p[m.0 as usize];
    new_p[m.0 as usize] = 0;
    (new_p, d * step)
}

#[derive(Eq, PartialEq)]
struct PosWithCost {
    pos: [u8; 23],
    cost: u32,
}

impl Ord for PosWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PosWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_cost(p0: &[u8; 23], p1: &[u8; 23]) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut seen = FxHashSet::<[u8; 23]>::default();
    queue.push(PosWithCost { pos: *p0, cost: 0 });
    while let Some(PosWithCost { pos, cost }) = queue.pop() {
        if pos == *p1 {
            // found
            return Some(cost);
        }
        // if we've already seen this position skip
        if seen.get(&pos).is_some() {
            continue;
        }
        seen.insert(pos);
        let moves = possible_moves(&pos);
        for m in moves {
            let (next, step_cost) = do_move(&pos, m);
            queue.push(PosWithCost {
                pos: next,
                cost: cost + step_cost,
            })
        }
    }
    None
}

fn parse(input: &str) -> [u8; 23] {
    let pre_rooms = input
        .lines()
        .skip(2)
        .map(|line| {
            line.chars()
                .filter(|c| *c != '#' && *c != ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // first 7 are top row, after each 4 for rooms. rooms are empty at start
    let mut start_pos = [
        0u8, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 3, 2, 0, 0, 2, 1, 0, 0, 1, 3, 0,
    ];
    // fills rooms
    for (i, line) in pre_rooms.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            start_pos[(j % 4) * 4 + i * 3 + 7] = *c as u8 - b'A' + 1;
        }
    }
    start_pos
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2021, 23);
    let start_pos = parse(&input);
    let final_pos = [
        0u8, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4,
    ];
    let p2 = shortest_path_cost(&start_pos, &final_pos).unwrap();
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_23_test.in");
        let start_pos = parse(&input);
        let final_pos = [
            0u8, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4,
        ];
        let p2 = shortest_path_cost(&start_pos, &final_pos).unwrap();
        assert_eq!(p2, 44169);
    }
}
