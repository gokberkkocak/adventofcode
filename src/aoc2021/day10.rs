pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 10);
    let p1 = part1(&input);
    println!("Part 1: {}", p1);
    let p2 = part2(&input);
    println!("Part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(get_line_score)
        .filter_map(|s| s.corrupted())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut v = input
        .lines()
        .map(get_line_score)
        .filter_map(|s| s.incomplete())
        .collect::<Vec<_>>();
    v.sort_unstable();
    v[v.len() / 2]
}

enum LineScore {
    Incomplete(usize),
    Corrupted(usize),
}

impl LineScore {
    fn corrupted(&self) -> Option<usize> {
        match self {
            LineScore::Corrupted(s) => Some(*s),
            _ => None,
        }
    }

    fn incomplete(&self) -> Option<usize> {
        match self {
            LineScore::Incomplete(s) => Some(*s),
            _ => None,
        }
    }
}

fn get_line_score(s: &str) -> LineScore {
    let mut stack = vec![];
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            '}' | ')' | ']' | '>' if Some(i) != stack.pop() => match i {
                ')' => return LineScore::Corrupted(3),
                ']' => return LineScore::Corrupted(57),
                '}' => return LineScore::Corrupted(1197),
                '>' => return LineScore::Corrupted(25137),
                _ => unreachable!(),
            },
            _ => (),
        }
    }
    if stack.is_empty() {
        LineScore::Corrupted(0)
    } else {
        let score = stack.iter().rev().fold(0, |acc, i| {
            match i {
                ')' => acc * 5 + 1,
                ']' => acc * 5 + 2,
                '}' => acc * 5 + 3,
                '>' => acc * 5 + 4,
                _ => unreachable!(),
            }
        });
        LineScore::Incomplete(score)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_10_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 26397);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_10_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 288957);
    }
}
