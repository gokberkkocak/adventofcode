use std::collections::BinaryHeap;

use fxhash::FxHashMap;

const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const BUF_WEIGHT: [i64; 7] = [0, 1, 3, 5, 7, 9, 10];

fn weight(c: char) -> i64 {
    WEIGHT[(c as u8 - b'A') as usize]
}

fn buf_traversal_cost(i: usize, j: usize, c: char) -> i64 {
    (BUF_WEIGHT[i] - BUF_WEIGHT[j]).abs() * WEIGHT[(c as u8 - b'A') as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    rooms: Vec<Vec<char>>,
    buffer: [char; 7],
    room_size: usize,
}

impl State {
    fn is_valid_room(&self, i: usize) -> bool {
        self.rooms[i]
            .iter()
            .all(|&c| i == (c as u8 - b'A') as usize)
    }

    fn entry_cost(&self, i: usize) -> i64 {
        (self.room_size - self.rooms[i].len()) as i64 * WEIGHT[i]
    }

    fn exit_cost(&self, i: usize, c: char) -> i64 {
        (self.room_size - self.rooms[i].len()) as i64 * weight(c)
    }

    fn transition_room_to_buffer(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..4 {
            if self.is_valid_room(i) {
                continue;
            }
            let mut next = self.clone();
            let c = next.rooms[i].pop().unwrap();
            for j in (0..=i + 1).rev() {
                let cost = buf_traversal_cost(j, i + 1, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == '.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = '.';
                } else {
                    break;
                }
            }
            for j in i + 2..7 {
                let cost = buf_traversal_cost(i + 2, j, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == '.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = '.';
                } else {
                    break;
                }
            }
        }
        res
    }

    fn transition_buffer_to_room(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..7 {
            if self.buffer[i] == '.' {
                continue;
            }
            let r = (self.buffer[i] as u8 - b'A') as usize;
            if !self.is_valid_room(r) {
                continue;
            }
            if i <= r + 1 {
                if (i + 1..=r + 1).all(|i| self.buffer[i] == '.') {
                    let mut next = self.clone();
                    let c = buf_traversal_cost(i, r + 1, next.buffer[i])
                        + weight(next.buffer[i])
                        + self.entry_cost(r);
                    next.rooms[r].push(next.buffer[i]);
                    next.buffer[i] = '.';
                    res.push((next, c));
                }
            } else if (r + 2..i).all(|i| self.buffer[i] == '.') {
                let mut next = self.clone();
                let c = buf_traversal_cost(r + 2, i, next.buffer[i])
                    + weight(next.buffer[i])
                    + self.entry_cost(r);
                next.rooms[r].push(next.buffer[i]);
                next.buffer[i] = '.';
                res.push((next, c));
            }
        }
        res
    }

    fn transitions(&self) -> Vec<(State, i64)> {
        let mut res = self.transition_room_to_buffer();
        res.append(&mut self.transition_buffer_to_room());
        res
    }
}

fn parse(input: &str, part2: bool) -> State {
    let pre_rooms = input
        .lines()
        .skip(2)
        .map(|line| {
            line.chars()
                .filter(|c| *c != '#' && *c != ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut rooms = vec![vec!['.'; 4]; 4];
    for (y, row) in pre_rooms.iter().enumerate().take(pre_rooms.len() - 1) {
        for (x, r) in rooms.iter_mut().enumerate().take(pre_rooms[y].len()) {
            r[pre_rooms[y].len() - 1 - y] = row[x];
        }
    }
    for r in rooms.iter_mut() {
        r.remove(0);
        r.remove(0);
    }
    if part2 {
        rooms[0].insert(1, 'D');
        rooms[0].insert(2, 'D');
        rooms[1].insert(1, 'B');
        rooms[1].insert(2, 'C');
        rooms[2].insert(1, 'A');
        rooms[2].insert(2, 'B');
        rooms[3].insert(1, 'C');
        rooms[3].insert(2, 'A');
    }
    let room_size = if part2 { 4 } else { 2 };
    State {
        rooms,
        buffer: ['.'; 7],
        room_size,
    }
}

pub fn part1(input: &str) -> i64 {
    let start = parse(input, false);
    let expected = State {
        rooms: vec![
            vec!['A', 'A'],
            vec!['B', 'B'],
            vec!['C', 'C'],
            vec!['D', 'D'],
        ],
        buffer: ['.'; 7],
        room_size: 2,
    };
    d(start, expected)
}

fn part2(input: &str) -> i64 {
    let start = parse(input, true);
    let expected = State {
        rooms: vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'B', 'B'],
            vec!['C', 'C', 'C', 'C'],
            vec!['D', 'D', 'D', 'D'],
        ],
        buffer: ['.'; 7],
        room_size: 4,
    };
    d(start, expected)
}

fn d(s: State, f: State) -> i64 {
    let mut costs = FxHashMap::default();
    let mut q = BinaryHeap::new();
    costs.insert(s.clone(), 0);
    q.push((0, s));
    while let Some((cost, grid)) = q.pop() {
        let cost = -cost;
        if cost != costs[&grid] {
            continue;
        }
        if grid == f {
            break;
        }
        for (transition, t_cost) in grid.transitions() {
            if let Some(&c) = costs.get(&transition) {
                if c <= t_cost + cost {
                    continue;
                }
            }
            costs.insert(transition.clone(), t_cost + cost);
            q.push((-(t_cost + cost), transition));
        }
    }
    *costs.get(&f).unwrap()
}

pub fn run() {
    let input = crate::util::get_puzzle_input(2021, 23);
    let p1 = part1(&input);
    println!("Part 1: {}", p1);
    let p2 = part2(&input);
    println!("Part 1: {}", p2);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_23_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 12521);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_23_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 44169);
    }
}
