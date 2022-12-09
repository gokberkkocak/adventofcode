pub fn run() {
    let (s, i) = parse(&crate::util::get_puzzle_input(2022, 5));
    let p1 = part1(s.clone(), &i);
    println!("p1: {}", p1);
    let p2 = part2(s, &i);
    println!("p2: {}", p2);
}

pub fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut iter = input.split("\n\n");
    let stack_input = iter.next().unwrap();
    let v = stack_input
        .lines()
        .rev()
        .skip(1)
        .flat_map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|chunk| chunk[1] as char)
                .enumerate()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let stacks = create_stacks(&v);

    let instructions = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0] as usize, v[1] as usize, v[2] as usize)) // quantity, from, to
        .collect::<Vec<_>>();
    (stacks, instructions)
}

pub fn create_stacks(v: &[(usize, char)]) -> Vec<Vec<char>> {
    let max_stack_n = v.iter().map(|(i, _)| i).max().unwrap();
    let mut stack = vec![vec![]; *max_stack_n + 1];
    for (i, c) in v {
        if *c <= 'Z' && *c >= 'A' {
            stack[*i].push(*c);
        }
    }
    stack
}

pub fn part1(mut stacks: Vec<Vec<char>>, instructions: &[(usize, usize, usize)]) -> String {
    for (quantity, from, to) in instructions {
        for _ in 0..*quantity {
            let tmp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(tmp);
        }
    }
    stacks
        .iter()
        .map(|v| v.iter().last().unwrap())
        .collect::<String>()
}

pub fn part2(mut stacks: Vec<Vec<char>>, instructions: &[(usize, usize, usize)]) -> String {
    let max_move = instructions.iter().map(|(q, _, _)| q).max().unwrap();
    let mut temp = vec!['0'; *max_move];
    for (quantity, from, to) in instructions {
        let len = stacks[from - 1].len();
        temp[..*quantity].clone_from_slice(&stacks[from - 1][len - quantity..len]);
        stacks[from - 1].truncate(len - quantity);
        stacks[to - 1].extend(temp.iter().take(*quantity));
    }
    stacks
        .iter()
        .map(|v| v.iter().last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_5_test.in");
        let (s, i) = parse(&input);
        let p1 = part1(s, &i);
        assert_eq!(p1, "CMZ");
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_5_test.in");
        let (s, i) = parse(&input);
        let p2 = part2(s, &i);
        assert_eq!(p2, "MCD");
    }
}
