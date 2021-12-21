pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 1);
    let v = parse(&input);
    let p1 = part1(&v);
    println!("Part 1: {} ", p1);
    let p2 = part2(&v);
    println!("Part 2: {} ", p2);
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(v: &[u32]) -> usize {
    part_core(v, 1)
}

fn part2(v: &[u32]) -> usize {
    part_core(v, 3)
}

fn part_core(v: &[u32], skip: usize) -> usize {
    v.iter()
        .zip(v.iter().skip(skip))
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_1_test.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 7);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_1_test.in");
        let v = parse(&input);
        let p2 = part2(&v);
        assert_eq!(p2, 5);
    }
}
