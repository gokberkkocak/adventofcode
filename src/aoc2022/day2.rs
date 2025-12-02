pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 2);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    println!("p1: {}", p2);
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|b| ((b[0] as u8 - b'A') as i32, (b[2] as u8 - b'X') as i32))
        .map(|(op, me)| 1 + me + 3 * (1 + me - op).rem_euclid(3))
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|b| ((b[0] as u8 - b'A') as i32, (b[2] as u8 - b'X') as i32))
        .map(|(op, win)| 1 + (op + win + 2) % 3 + 3 * win)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_2_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 15);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_2_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 12);
    }
}
