pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 18);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

struct ArithmeticState<'a> {
    formula: &'a str,
    result: isize,
    current_operator: Operator,
    parse_status: ParseStatus,
}

impl<'a> ArithmeticState<'a> {
    fn new(formula: &'a str) -> Self {
        Self {
            formula,
            result: 0,
            current_operator: Operator::Sum,
            parse_status: ParseStatus::Operator,
        }
    }

    fn calculate(&mut self) -> (isize, usize) {
        let mut i = 0;
        while i < self.formula.len() {
            let c = self.formula.chars().nth(i).unwrap();
            match c {
                c if c.is_alphanumeric() => match self.parse_status {
                    ParseStatus::Operator => {
                        self.parse_status = ParseStatus::Number(i, i + 1);
                    }
                    ParseStatus::Number(x, _) => {
                        self.parse_status = ParseStatus::Number(x, i + 1);
                    }
                },
                c if c == '+' => match self.parse_status {
                    ParseStatus::Operator => {
                        self.current_operator = Operator::Sum;
                    }
                    ParseStatus::Number(x, y) => {
                        self.parse_status = ParseStatus::Operator;
                        let num = self.formula[x..y].parse::<isize>().unwrap();
                        match self.current_operator {
                            Operator::Sum => self.result += num,
                            Operator::Multiply => self.result *= num,
                        }
                        self.current_operator = Operator::Sum;
                    }
                },
                c if c == '*' => match self.parse_status {
                    ParseStatus::Operator => {
                        self.current_operator = Operator::Multiply;
                    }
                    ParseStatus::Number(x, y) => {
                        self.parse_status = ParseStatus::Operator;
                        let num = self.formula[x..y].parse::<isize>().unwrap();
                        match self.current_operator {
                            Operator::Sum => self.result += num,
                            Operator::Multiply => self.result *= num,
                        }
                        self.current_operator = Operator::Multiply;
                    }
                },
                c if c == '(' => {
                    let mut sub_state = ArithmeticState::new(&self.formula[i + 1..]);
                    let (num, ind) = sub_state.calculate();
                    match self.current_operator {
                        Operator::Sum => self.result += num,
                        Operator::Multiply => self.result *= num,
                    }

                    i += ind + 1;
                }
                c if c == ')' => {
                    match self.parse_status {
                        ParseStatus::Operator => (),
                        ParseStatus::Number(x, y) => {
                            self.parse_status = ParseStatus::Operator;
                            let num = self.formula[x..y].parse::<isize>().unwrap();
                            match self.current_operator {
                                Operator::Sum => self.result += num,
                                Operator::Multiply => self.result *= num,
                            }
                        }
                    }
                    return (self.result, i);
                }
                c if c.is_ascii_whitespace() => (),
                _ => unreachable!(),
            }
            i += 1;
        }
        // if last one was Number
        match self.parse_status {
            ParseStatus::Number(x, y) => {
                let num = self.formula[x..y].parse::<isize>().unwrap();
                match self.current_operator {
                    Operator::Sum => self.result += num,
                    Operator::Multiply => self.result *= num,
                }
            }
            ParseStatus::Operator => (),
        }
        (self.result, i)
    }
}

enum Operator {
    Sum,
    Multiply,
}

enum ParseStatus {
    Number(usize, usize),
    Operator,
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut a = ArithmeticState::new(line);
            let (r, _) = a.calculate();
            r
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let new_line = add_parantheses(line);
            let mut a = ArithmeticState::new(&new_line);
            let (r, _) = a.calculate();
            r
        })
        .sum()
}

fn add_parantheses(input: &str) -> String {
    format!(
        "({})",
        input
            .replace("(", "((")
            .replace(")", "))")
            .replace(" * ", ") * (")
            .replace(" ", "")
    )
    .chars()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut a = ArithmeticState::new(input);
        let (r, _) = a.calculate();
        assert_eq!(r, 71);
    }

    #[test]
    fn test_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut a = ArithmeticState::new(input);
        let (r, _) = a.calculate();
        assert_eq!(r, 51);
    }
    #[test]
    fn test_3() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut a = ArithmeticState::new(input);
        let (r, _) = a.calculate();
        assert_eq!(r, 13632);
    }

    #[test]
    fn test_11() {
        let input = add_parantheses("1 + 2 * 3 + 4 * 5 + 6");
        let mut a = ArithmeticState::new(&input);
        let (r, _) = a.calculate();
        assert_eq!(r, 231);
    }

    #[test]
    fn test_12() {
        let input = add_parantheses("1 + (2 * 3) + (4 * (5 + 6))");
        let mut a = ArithmeticState::new(&input);
        let (r, _) = a.calculate();
        assert_eq!(r, 51);
    }
    #[test]
    fn test_13() {
        let input = add_parantheses("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        let mut a = ArithmeticState::new(&input);
        let (r, _) = a.calculate();
        assert_eq!(r, 23340);
    }
}
