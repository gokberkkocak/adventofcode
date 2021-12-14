pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 13);
    let mut ins = parse(&input);
    let p1 = part1(&mut ins);
    println!("Part 1: {}", p1);
    let p2 = part2(&mut ins);
    println!("Part 2:\n{}", p2);
}

fn part1(ins: &mut Instruction) -> usize {
    ins.apply_first_fold_on_stack();
    ins.points.len()
}

fn part2(ins: &mut Instruction) -> String {
    ins.apply_all_folds();
    format!("{}", ins)
}

fn parse(input: &str) -> Instruction {
    let mut it = input.split("\n\n");
    let points = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_it = line.split(',').map(|s| s.trim().parse::<usize>().unwrap());
            let x = line_it.next().unwrap();
            let y = line_it.next().unwrap();
            Point::new(x, y)
        })
        .collect::<Vec<_>>();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let max_point = Point::new(max_x, max_y);
    let fold_stack = it
        .next()
        .unwrap()
        .lines()
        .map(Fold::from)
        .rev()
        .collect::<Vec<_>>();
    Instruction {
        points,
        max_point,
        fold_stack,
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

enum Fold {
    OnY(usize), // horizontal line
    OnX(usize), // vertical line
}

impl From<&str> for Fold {
    fn from(s: &str) -> Self {
        let mut it = s.strip_prefix("fold along ").unwrap().split('=');
        let axis = it.next().unwrap();
        let value = it.next().unwrap().parse::<usize>().unwrap();
        match axis {
            "x" => Fold::OnX(value),
            "y" => Fold::OnY(value),
            _ => panic!("invalid axis"),
        }
    }
}

struct Instruction {
    points: Vec<Point>,
    max_point: Point,
    fold_stack: Vec<Fold>,
}

impl Instruction {
    fn apply_fold(&mut self, fold: Fold) {
        let mut max_point = self.max_point;
        match fold {
            Fold::OnY(y) => {
                self.points
                    .iter_mut()
                    .filter(|p| p.y > y)
                    .for_each(|p| p.y = max_point.y - p.y);
                max_point.y = y - 1;
            }
            Fold::OnX(x) => {
                self.points
                    .iter_mut()
                    .filter(|p| p.x > x)
                    .for_each(|p| p.x = max_point.x - p.x);
                max_point.x = x - 1;
            }
        }
        self.max_point = max_point;
        self.points.sort_unstable();
        self.points.dedup();
    }
    #[inline]
    fn apply_all_folds(&mut self) {
        while let Some(fold) = self.fold_stack.pop() {
            self.apply_fold(fold);
        }
    }
    #[inline]
    fn apply_first_fold_on_stack(&mut self) {
        let fold = self.fold_stack.pop().unwrap();
        self.apply_fold(fold);
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..=self.max_point.y {
            for x in 0..=self.max_point.x {
                if self.points.contains(&Point::new(x, y)) {
                    s.push('█');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_13_test.in");
        let mut ins = parse(&input);
        let p1 = part1(&mut ins);
        assert_eq!(p1, 17);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_13_test.in");
        let mut ins = parse(&input);
        let p2 = part2(&mut ins);
        let res = "█████\n\
                        █...█\n\
                        █...█\n\
                        █...█\n\
                        █████\n\
                        .....\n\
                        .....\n";
        assert_eq!(p2, res);
    }
}
