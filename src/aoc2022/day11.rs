pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 11);
    let mut v = parse(&input);
    let mut v_clone = v.clone();
    let p1 = play_monkey_business(&mut v, 20, true);
    println!("p1: {}", p1);
    let p2 = play_monkey_business(&mut v_clone, 10_000, false);
    println!("p2: {}", p2);
}

fn play_monkey_business(v: &mut [Monkey], turn: usize, boring: bool) -> usize {
    let big_divider = v.iter().map(|m| m.test.divisible_test).product::<usize>();
    for _ in 0..turn {
        play_turn(v, big_divider, boring);
    }
    let mut c = v.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    c.sort_unstable();
    c.iter().rev().take(2).product()
}

fn play_turn(v: &mut [Monkey], big_divider: usize, boring: bool) {
    for i in 0..v.len() {
        while let Some(item) = v[i].items.pop() {
            let mut adjusted = v[i].operation.apply(item % big_divider);
            if boring {
                adjusted /= 3;
            }
            let new_owner = v[i].test.test(adjusted);
            v[i].inspect_count += 1;
            v[new_owner].items.push(adjusted);
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut v = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    v.sort_unstable();
    v
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u8,
    items: Vec<usize>,
    operation: Operation,
    test: DivisibleTest,
    inspect_count: usize,
}

impl Monkey {
    fn parse(m_str: &str) -> Self {
        let mut lines = m_str.lines();
        let id = lines.next().unwrap().split(' ').nth(1).unwrap().as_bytes()[0] - b'0';
        let items = lines.next().unwrap().split(": ").nth(1).unwrap();
        let items = items
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let operation = Operation::parse(lines.next().unwrap());
        let test_line = lines.next().unwrap();
        let success_line = lines.next().unwrap();
        let fail_line = lines.next().unwrap();
        let test = DivisibleTest::new(test_line, success_line, fail_line);
        Monkey {
            id,
            items,
            operation,
            test,
            inspect_count: 0,
        }
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Monkey {}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn parse(op_str: &str) -> Self {
        let mut it = op_str.split("new = old ");
        let op = it.nth(1).unwrap();
        let op_char = op.as_bytes()[0];
        match op_char {
            b'+' => {
                let num = op.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
                Operation::Add(num)
            }
            b'*' => {
                if let Ok(num) = op.split(' ').nth(1).unwrap().parse::<usize>() {
                    Operation::Multiply(num)
                } else {
                    Operation::Square
                }
            }
            _ => unreachable!(),
        }
    }

    fn apply(&self, old: usize) -> usize {
        match self {
            Operation::Add(num) => old + num,
            Operation::Multiply(num) => old * num,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct DivisibleTest {
    divisible_test: usize,
    success_id: usize,
    failure_id: usize,
}

impl DivisibleTest {
    fn new(test_line: &str, success_line: &str, fail_line: &str) -> Self {
        let divisible_test = test_line
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let success_id = success_line
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let failure_id = fail_line
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        DivisibleTest {
            divisible_test,
            success_id,
            failure_id,
        }
    }

    fn test(&self, num: usize) -> usize {
        if num % self.divisible_test == 0 {
            self.success_id
        } else {
            self.failure_id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_11_test.in");
        let mut v = parse(&input);
        let p1 = play_monkey_business(&mut v, 20, true);
        assert_eq!(p1, 10605);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_11_test.in");
        let mut v = parse(&input);
        let p2 = play_monkey_business(&mut v, 10_000, false);
        assert_eq!(p2, 2713310158);
    }
}
