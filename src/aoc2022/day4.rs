pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 4);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    println!("p2: {}", p2);
}

pub fn parse(input: &str) -> impl Iterator<Item = (u8, u8, u8, u8)> + '_ {
    input.lines().map(|pair_str| {
        let pairs = pair_str.split(',').collect::<Vec<_>>();
        let elf_1 = pairs[0]
            .split('-')
            .map(|i| i.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let elf_2 = pairs[1]
            .split('-')
            .map(|i| i.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        (elf_1[0], elf_1[1], elf_2[0], elf_2[1])
    })
}

pub fn part1(input: &str) -> u64 {
    parse(&input)
        .filter(|(a, b, m, n)| (m <= a && b <= n) || (a <= m && n <= b))
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    parse(&input)
        .filter(|(a, b, m, n)| (a <= m && m <= b) || (m <= a && a <= n))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_4_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 2);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_4_test.in");
        let p1 = part2(&input);
        assert_eq!(p1, 4);
    }
}
