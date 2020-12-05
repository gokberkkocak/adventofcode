use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 5);
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("{}", p1);
    println!("{}", p2);
}

pub fn part1(input: &str) -> usize {
    input.lines().map(|x| get_seat_value(x)).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let max = input.lines().map(|x| get_seat_value(x)).max().unwrap();
    let min = input.lines().map(|x| get_seat_value(x)).min().unwrap();
    let sum = input.lines().map(|x| get_seat_value(x)).sum::<usize>();
    (max * (max + 1) / 2) - sum - ((min - 1) * min / 2)
}

fn get_seat_value(entry: &str) -> usize {
    let (row_str, col_str) = entry.split_at(7);
    let row = row_str
        .chars()
        .rev()
        .enumerate()
        .filter(|(_i, c)| *c == 'B')
        .map(|(i, _c)| usize::pow(2, i as u32))
        .sum::<usize>();
    let col = col_str
        .chars()
        .rev()
        .enumerate()
        .filter(|(_i, c)| *c == 'R')
        .map(|(i, _c)| usize::pow(2, i as u32))
        .sum::<usize>();
    let seat = row * 8 + col;
    seat
}
