pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 10);
    let p1 = part1(&input);
    println!("p1: {}", p1);
    let p2 = part2(&input);
    for line in p2 {
        println!("{}", line);
    }
}

fn parse(input: &str) -> Vec<Op> {
    input.lines().map(Op::new).collect()
}

enum Op {
    Add(i32),
    Noop,
}

impl Op {
    fn new(line: &str) -> Self {
        let mut it = line.split_ascii_whitespace();
        let op = it.next().unwrap();
        match op {
            "addx" => Self::Add(it.next().unwrap().parse::<i32>().unwrap()),
            _ => Self::Noop,
        }
    }
}

struct Instructions(Vec<(i32, i32)>);

impl Instructions {
    fn new(ops: Vec<Op>) -> Self {
        let mut cycles = 0;
        let mut x = 1;
        Self(
            ops.iter()
                .map(|op| {
                    match op {
                        Op::Add(amount) => {
                            x += amount;
                            cycles += 2;
                        }
                        Op::Noop => cycles += 1,
                    }
                    (cycles, x)
                })
                .collect::<Vec<_>>(),
        )
    }
}

const WANTED: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn part1(input: &str) -> i32 {
    let ops = parse(input);
    let v = Instructions::new(ops).0;
    let mut wanted_signals = [0; 6];
    for i in 0..v.len() - 1 {
        let (c1, x1) = v[i];
        let (c2, _) = v[i + 1];
        for j in 0..WANTED.len() {
            if c1 <= WANTED[j] && WANTED[j] <= c2 && wanted_signals[j] == 0 {
                wanted_signals[j] = x1 * WANTED[j];
                break;
            }
        }
    }
    wanted_signals.iter().sum()
}

fn part2(input: &str) -> Vec<String> {
    let ops = parse(input);
    let v = Instructions::new(ops).0;
    let mut display = vec![vec!['.'; 40]; 6];
    let mut register_prev = 1i32;
    let mut c_prev = 0;
    for (c, register) in v {
        for i in c_prev..c {
            let y = i / 40;
            let x = i % 40;
            if register_prev - 1 <= x && x <= register_prev + 1 {
                display[y as usize][x as usize] = '#';
            }
        }
        c_prev = c;
        register_prev = register;
    }

    display
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_10_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 13140);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_10_test.in");
        let p2 = part2(&input);
        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        assert_eq!(p2, expected);
    }
}
