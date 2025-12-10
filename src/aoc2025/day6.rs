pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 6);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    let batches = parse_input_naive(input);
    batches.iter().map(|batch| batch.calculate()).sum()
}

fn part2(input: &str) -> usize {
    let batches = parse_input_transpose(input);
    batches.iter().map(|batch| batch.calculate()).sum()
}

fn parse_input_naive(input: &str) -> Vec<Batch> {
    let nb_batches = input.lines().next().unwrap().split_whitespace().count();
    let mut batches = vec![Batch::new_empty(); nb_batches];

    input.lines().rev().skip(1).for_each(|line| {
        line.split_ascii_whitespace()
            .zip(batches.iter_mut())
            .for_each(|(part, batch)| {
                let num: usize = part.parse().unwrap();
                batch.items.push(num);
            })
    });

    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .zip(batches.iter_mut())
        .for_each(|(part, batch)| match part {
            "*" => batch.operation = Operation::Multiply,
            "+" => batch.operation = Operation::Add,
            _ => panic!("Unknown operation"),
        });
    batches
}

// quick dirty solution using transposition of the input grid
fn parse_input_transpose(input: &str) -> Vec<Batch> {
    let line_len = input.lines().next().unwrap().len();
    let mut line_iterator = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|n| n.chars().into_iter())
        .collect::<Vec<_>>();

    let transposed = (0..line_len)
        .map(|_| {
            line_iterator
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // read transposed grid and construct batches
    let nb_batches = input.lines().next().unwrap().split_whitespace().count();
    let mut batches = vec![Batch::new_empty(); nb_batches];
    let mut current_batch_idx = 0;

    for row in transposed {
        let line: String = row.into_iter().collect();
        if line.trim().is_empty() {
            current_batch_idx += 1;
        } else {
            batches[current_batch_idx]
                .items
                .push(line.trim().parse().unwrap());
        }
    }

    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .zip(batches.iter_mut())
        .for_each(|(part, batch)| match part {
            "*" => batch.operation = Operation::Multiply,
            "+" => batch.operation = Operation::Add,
            _ => panic!("Unknown operation"),
        });
    batches
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Add,
    None,
}
#[derive(Debug, Clone)]
struct Batch {
    operation: Operation,
    items: Vec<usize>,
}

impl Batch {
    fn new_empty() -> Self {
        Self {
            operation: Operation::None,
            items: vec![],
        }
    }

    fn calculate(&self) -> usize {
        match self.operation {
            Operation::Multiply => self.items.iter().product(),
            Operation::Add => self.items.iter().sum(),
            Operation::None => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_6_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 4277556);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_6_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 3263827);
    }
}
