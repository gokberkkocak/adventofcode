pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 2);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    let product_ranges = get_product_ranges(input);
    product_ranges
        .iter()
        .map(|(start, end)| (*start..=*end).filter(|n| p1_invalid(*n)).sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    let product_ranges = get_product_ranges(input);
    product_ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end).filter(|n| p2_invalid(*n)).sum::<usize>()
        })
        .sum()
}

fn get_product_ranges(input: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    for pair in input.split(",") {
        let parts = pair.split("-").collect::<Vec<&str>>();
        let first = parts[0].parse().unwrap();
        let last = parts[1].parse().unwrap();
        ranges.push((first, last));
    }
    ranges
}

fn p1_invalid(n: usize) -> bool {
    let s = n.to_string();
    let total_length = s.len();
    if total_length % 2 != 0 {
        return false;
    }
    let half = total_length / 2;
    s[..half] == s[half..]
}

fn p2_invalid(n: usize) -> bool {
    let s = n.to_string();
    let total_length = s.len();
    let half_length = total_length / 2;
    for length in 1..=half_length {
        if total_length % length != 0 {
            continue;
        }
        let repeat = total_length / length;
        if s[..length].repeat(repeat) == s {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_2_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 1227775554);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_2_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 4174379265);
    }
}
