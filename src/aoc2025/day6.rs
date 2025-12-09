pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 6);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

fn part1(input: &str) -> usize{
    let batches = parse_input_naive(input);
    batches.iter().map(|batch| batch.calculate()).sum()
}

fn part2(input: &str) -> usize {
    let batches = parse_input_transpose(input);
    batches.iter().map(|batch| batch.calculate()).sum()
}

fn parse_input_naive(input: &str) -> Vec<Batch> {
    let len = input.lines().next().unwrap().split_whitespace().count();
    let mut batches = vec![Batch::new_empty(); len];

    for line in input.lines().take(input.lines().count() - 1) {
        line.split_ascii_whitespace()
            .zip(batches.iter_mut())
            .for_each(|(part, batch)| {
                let num: usize = part.parse().unwrap();
                batch.items.push(num);
            });
    }
    let last_line = input.lines().last().unwrap();
    last_line
        .split_ascii_whitespace()
        .zip(batches.iter_mut())
        .for_each(|(part, batch)| match part {
            "*" => batch.operation = Operation::Multiply,
            "+" => batch.operation = Operation::Add,
            _ => panic!("Unknown operation"),
        });
    batches
}

// quick dirty solution in less than 10 minutes
fn parse_input_transpose(input: &str) -> Vec<Batch> {
    // put every char in a 2d char array except last line
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines().take( input.lines().count() - 1 ) {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    // transpose the grid
    let mut transposed: Vec<Vec<char>> = vec![];
    for col in 0..grid[0].len() {
        let mut new_row: Vec<char> = vec![];
        for row in 0..grid.len() {
            new_row.push(grid[row][col]);
        }
        transposed.push(new_row);
    }

    // convert transposed into multi line string again
    let mut transposed_str = String::new();
    for row in transposed {
        let line: String = row.into_iter().collect();
        if line.trim().is_empty() {
            transposed_str.push('\n');
        } else {
            transposed_str.push_str(&line);
            transposed_str.push(' ');
        }
    }
    let mut batches = vec![Batch::new_empty(); transposed_str.lines().count()];
    for (line, batch) in transposed_str.lines().zip(batches.iter_mut()) {
        batch.items = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();   
    }

    let last_line = input.lines().last().unwrap();
    last_line
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
