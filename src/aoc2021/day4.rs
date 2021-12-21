use core::fmt;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 4);
    let (drawn, mut boards) = parse(&input);
    let mut cloned_boards = boards.clone();
    let p1 = part1(&drawn, &mut boards);
    println!("Part 1: {}", p1);
    let p2 = part2(&drawn, &mut cloned_boards);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let drawn = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let boards = input
        .split("\n\n")
        .skip(1)
        .map(Board::from)
        .collect::<Vec<_>>();
    (drawn, boards)
}

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<BingoValue>>,
    done: bool,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum BingoValue {
    AlreadyDrawn(usize),
    NotDrawn(usize),
}

impl BingoValue {
    fn is_drawn(&self) -> bool {
        match self {
            BingoValue::AlreadyDrawn(_) => true,
            BingoValue::NotDrawn(_) => false,
        }
    }
}

impl fmt::Display for BingoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BingoValue::AlreadyDrawn(d) => write!(f, "A{}", d),
            BingoValue::NotDrawn(d) => write!(f, "N{}", d),
        }
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|b| BingoValue::NotDrawn(b.parse::<usize>().unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Board { rows, done: false }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for bingo in row {
                write!(f, "{} ", bingo)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn apply_drawn(&mut self, d: usize) {
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|bv| {
                if let BingoValue::NotDrawn(n) = bv {
                    if *n == d {
                        *bv = BingoValue::AlreadyDrawn(*n);
                    }
                }
            })
        });
    }

    fn check_win(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(|bv| bv.is_drawn()))
            || (0..self.rows.len())
                .any(|column| (0..self.rows.len()).all(|r| self.rows[r][column].is_drawn()))
    }

    fn set_done(&mut self) {
        self.done = true;
    }

    fn already_done(&self) -> bool {
        self.done
    }

    fn sum_of_unmarked(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|bv| match bv {
                        BingoValue::NotDrawn(n) => *n,
                        BingoValue::AlreadyDrawn(_) => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

fn part1(drawn: &[usize], boards: &mut [Board]) -> usize {
    for i in drawn {
        boards.iter_mut().for_each(|b| b.apply_drawn(*i));
        if let Some(winner) = boards.iter().find(|b| b.check_win()) {
            return winner.sum_of_unmarked() * i;
        }
    }
    panic!("No win")
}

fn part2(drawn: &[usize], boards: &mut [Board]) -> usize {
    let board_len = boards.len();
    let mut count = 0;
    for i in drawn {
        boards.iter_mut().for_each(|b| b.apply_drawn(*i));
        while let Some(winner) = boards
            .iter_mut()
            .find(|b| !b.already_done() && b.check_win())
        {
            winner.set_done();
            count += 1;
            if count == board_len {
                return winner.sum_of_unmarked() * i;
            }
        }
    }
    panic!("No win")
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_4_test.in");
        let (d, mut boards) = parse(&input);
        let p1 = part1(&d, &mut boards);
        assert_eq!(p1, 4512);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_4_test.in");
        let (d, mut boards) = parse(&input);
        let p2 = part2(&d, &mut boards);
        assert_eq!(p2, 1924);
    }
}
