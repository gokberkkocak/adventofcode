use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 3);
    println!("p1 {}", part1(&input));
    println!("p2 {}", part2(&input));
}

fn tree_count(forest: &str, r_step: usize, d_step: usize) -> usize {
    forest
        .lines()
        .step_by(d_step)
        .enumerate()
        .filter(|(i, x)| x.chars().nth(i * r_step % x.len()) == Some('#'))
        .count()
}


fn part1(forest: &str) -> usize {
    tree_count(forest, 3, 1)
}

fn part2(forest: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (r, d)| acc * tree_count(forest, *r, *d))
}