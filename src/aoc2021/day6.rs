pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 6);
    let mut l = parse(&input);
    let p1 = solve(&mut l, 80);
    println!("Part 1: {}", p1);
    let p2 = solve(&mut l, 256 - 80);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> LanternFishes {
    LanternFishes::new(input.split(",").map(|i| i.parse().unwrap()).collect())
}

struct LanternFishes {
    timers: Vec<usize>,
}

impl LanternFishes {
    fn new(v: Vec<usize>) -> Self {
        let mut timers = vec![0; 9];
        for i in v {
            timers[i] += 1;
        }
        Self { timers }
    }
    fn apply_day(&mut self) {
        self.timers.rotate_left(1);
        self.timers[6] += self.timers[8];
    }
}

fn solve(l: &mut LanternFishes, days: usize) -> usize {
    for _ in 0..days {
        l.apply_day()
    }
    l.timers.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_6_test.in");
        let mut l = parse(&input);
        let p1 = solve(&mut l, 18);
        assert_eq!(p1, 26);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_6_test.in");
        let mut l = parse(&input);
        let p1 = solve(&mut l, 80);
        assert_eq!(p1, 5934);
    }
    #[test]
    fn test_3() {
        let input = crate::util::read_file("inputs/2021_6_test.in");
        let mut l = parse(&input);
        let p1 = solve(&mut l, 256);
        assert_eq!(p1, 26984457539);
    }
}
