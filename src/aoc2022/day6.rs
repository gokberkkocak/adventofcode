pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 6);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    println!("p2: {}", p2);
}

pub fn core(input: &str, d: usize) -> usize {
    input
        .as_bytes()
        .windows(d)
        .enumerate()
        .find(|(_, w)| {
            w.iter()
                .enumerate()
                .all(|(i, x)| w.iter().skip(i + 1).find(|&y| x == y).is_none())
        })
        .unwrap()
        .0
        + d
}

pub fn part1(input: &str) -> usize {
    core(input, 4)
}

pub fn part2(input: &str) -> usize {
    core(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part1(input), 7);
    }
    #[test]
    fn test_1_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_1_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_1_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(input), 10);
    }
    #[test]
    fn test_1_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_2_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part2(input), 19);
    }
    #[test]
    fn test_2_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part2(input), 23);
    }

    #[test]
    fn test_2_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part2(input), 23);
    }

    #[test]
    fn test_2_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part2(input), 29);
    }
    #[test]
    fn test_2_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part2(input), 26);
    }
}
