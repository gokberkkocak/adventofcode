pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 3);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

// optimised from initial version and switched to greedy algorithm with reddit influence
fn find_battery(input: &str, nb_digits: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let mut max = 0;
            let mut start = 0;

            (0..nb_digits).fold(0, |acc_joltage, digit| {
                // we want to greedily pick the max digit while leaving enough space for the remaining digits
                let end = line.len() - nb_digits + digit + 1;

                // we find our max and set the new search start point for the next digit
                (max, start) = (start..end).fold((0, 0), |(max, start), i| {
                    if line.as_bytes()[i] > max {
                        (line.as_bytes()[i], i + 1)
                    } else {
                        (max, start)
                    }
                });
                // dbg!(max - b'0', start, end, digit);
                10 * acc_joltage + (max - b'0') as usize
            })
        })
        .sum()
}

fn part1(input: &str) -> usize {
    find_battery(input, 2)
}

fn part2(input: &str) -> usize {
    find_battery(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_3_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 357);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_3_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 3121910778619);
    }
}
