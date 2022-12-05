pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 3);
    let p1 = part1(&input);
    println!("Part 1: {}", p1);
    let p2 = part2(&input);
    println!("Part 2: {}", p2);
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char_to_priority(c))
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let n = line.len();
            let r1 = line[..n / 2].iter().collect::<Vec<_>>();
            let r2 = line[n / 2..].iter().collect::<Vec<_>>();
            let el = **r1.iter().find(|c| r2.contains(c)).unwrap();
            el as u64
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char_to_priority(c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            *chunk[0]
                .iter()
                .find(|&c| chunk[1].contains(c) && chunk[2].contains(c))
                .unwrap() as u64
        })
        .sum()
}

pub fn char_to_priority(c: char) -> u8 {
    if c as u8 >= b'a' {
        (c as u8 - b'a') + 1
    } else {
        (c as u8 - b'A') + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_3_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 157);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_3_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 70);
    }
}
