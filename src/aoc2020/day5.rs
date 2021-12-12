use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 5);
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1 {}", p1);
    println!("p1 {}", p2);
}

pub fn part1(input: &str) -> usize {
    input.lines().map(get_seat_value).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let seats = input.lines().map(get_seat_value).collect::<Vec<_>>();
    let max = seats.iter().max().unwrap();
    let min = seats.iter().min().unwrap();
    let sum = seats.iter().sum::<usize>();
    (max * (max + 1) / 2) - sum - ((min - 1) * min / 2)
}

fn get_seat_value(entry: &str) -> usize {
    fn get_seat_inner_value(value: &str, bit_indicator: char) -> usize {
        value
            .chars()
            .rev()
            .enumerate()
            .filter(|(_i, c)| *c == bit_indicator)
            .map(|(i, _c)| usize::pow(2, i as u32))
            .sum::<usize>()
    }

    let (row_str, col_str) = entry.split_at(7);
    let row = get_seat_inner_value(row_str, 'B');
    let col = get_seat_inner_value(col_str, 'R');
    row * 8 + col
}
