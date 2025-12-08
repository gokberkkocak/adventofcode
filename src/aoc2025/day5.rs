pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 5);
    let (ranges, values) = parse_input(&input);
    let p1 = part1(&ranges, &values);
    println!("part 1: {}", p1);
    let p2 = part2(&ranges);
    println!("part 2: {}", p2);
}

fn part1(ranges: &[std::ops::Range<usize>], values: &[usize]) -> usize {
    // optimised with binary search 
    let fn_position = |id| values.binary_search(&id).unwrap_or_else(|e| e);
    ranges
        .iter()
        .map(|range| {
            dbg!(range, fn_position(range.end), fn_position(range.start));
            fn_position(range.end) - fn_position(range.start)
        })
        .sum()
}

fn part2(ranges: &[std::ops::Range<usize>]) -> usize {
    ranges.iter().map(|range| range.end - range.start).sum()
}

fn parse_input(input: &str) -> (Vec<std::ops::Range<usize>>, Vec<usize>) {
    let (mut ranges, mut values) = parse(input);
    ranges.sort_unstable();
    values.sort_unstable();
    let mut range = 0..0;
    let mut merged = Vec::new();
    for [from, to] in ranges {
        if from <= range.end {
            range.end = range.end.max(to + 1);
        } else {
            merged.push(range);
            range = from..to + 1;
        }
    }
    merged.push(range);
    (merged, values)
}

fn parse(input: &str) -> (Vec<[usize; 2]>, Vec<usize>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let ranges = parts[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();
            [start, end]
        })
        .collect();
    let values = parts[1]
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (ranges, values)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_5_test.in");
        let (ranges, values) = parse_input(&input);
        let p1 = part1(&ranges, &values);
        assert_eq!(p1, 3);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_5_test.in");
        let (ranges, _values) = parse_input(&input);
        let p2 = part2(&ranges);
        assert_eq!(p2, 14);
    }
}
