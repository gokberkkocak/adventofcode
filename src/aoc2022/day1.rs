pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 1);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

pub fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|n| n.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut carried = input
        .split("\n\n")
        .map(|e| e.lines().map(|n| n.parse::<u64>().unwrap()).sum())
        .collect::<Vec<_>>();
    carried.sort_unstable();
    carried.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_1_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 24000);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_1_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 45000);
    }
}
