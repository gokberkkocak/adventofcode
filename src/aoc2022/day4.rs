pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 4);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    println!("p2: {}", p2);
}

pub fn parse(input: &str) -> impl Iterator<Item = (u8, u8, u8, u8)> + '_ {
    input
        .lines()
        .map(|pair| {
            pair
                .split(',')
                .flat_map(|a| a.split('-').map(|i| i.parse::<u8>().unwrap()))
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2], v[3]))
}

pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|(a, b, m, n)| (m <= a && b <= n) || (a <= m && n <= b))
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|(a, b, m, n)| m <= b && a <= n)
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
