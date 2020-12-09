use std::collections::HashSet;

use crate::util::get_puzzle_input;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\w+) ([-+]\d+)").unwrap();
}
#[derive(Debug, Clone)]
struct Instruction<'a>(&'a str, isize);

#[derive(Debug, Clone)]
struct State<'a> {
    current_line: isize,
    acc: isize,
    executed_lines: HashSet<isize>,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> State<'a> {
    pub fn new(input: &'a str) -> Self {
        let instructions = input
            .lines()
            .map(|x| {
                let matches = REG.captures(x).unwrap();
                let instruction = matches.get(1).unwrap().as_str();
                let value = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Instruction(instruction, value)
            })
            .collect::<Vec<Instruction>>();
        Self {
            current_line: 0,
            acc: 0,
            executed_lines: HashSet::new(),
            instructions,
        }
    }
    pub fn execute_all(&mut self) {
        while (self.current_line as usize) < self.instructions.len()
            && !self.executed_lines.contains(&self.current_line)
        {
            self.execute_next();
        }
    }
    fn execute_next(&mut self) {
        self.executed_lines.insert(self.current_line);
        let Instruction(instruction, value) = self.instructions[self.current_line as usize];
        match instruction {
            "acc" => {
                self.acc += value;
                self.current_line += 1;
            }
            "nop" => self.current_line += 1,
            "jmp" => self.current_line += value,
            _ => unreachable!(),
        }
    }

    fn is_naturally_terminated(&self) -> bool {
        self.current_line as usize == self.instructions.len()
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.current_line = 0;
        self.executed_lines = HashSet::new();
    }
}

pub fn run() {
    let input = get_puzzle_input(2020, 8);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

fn part1(input: &str) -> isize {
    let mut state = State::new(input);
    state.execute_all();
    state.acc
}

fn part2(input: &str) -> isize {
    let init_state = State::new(input);
    let mut result_acc = 0;
    init_state
        .instructions
        .iter()
        .enumerate()
        .filter(|(_i, &Instruction(ins, _value))| ins == "nop" || ins == "jmp")
        .take_while(|&(i, _ins)| {
            let mut clone_state = init_state.clone();
            if clone_state.instructions[i].0 == "nop" {
                clone_state.instructions[i].0 = "jmp";
            } else {
                clone_state.instructions[i].0 = "nop";
            }
            clone_state.execute_all();
            result_acc = clone_state.acc;
            !clone_state.is_naturally_terminated()
        })
        .count();
    result_acc
}

fn part2_no_clone(input: &str) -> isize {
    let mut state = State::new(input);
    let mut result_acc = 0;
    for i in 0..state.instructions.len() {
        if state.instructions[i].0 == "acc" {
            continue;
        } else if state.instructions[i].0 == "nop" {
            state.instructions[i].0 = "jmp";
        } else {
            state.instructions[i].0 = "nop";
        }
        state.execute_all();
        if state.is_naturally_terminated() {
            result_acc = state.acc;
            break;
        }
        if state.instructions[i].0 == "nop" {
            state.instructions[i].0 = "jmp";
        } else {
            state.instructions[i].0 = "nop";
        }
        state.reset();
    }
    result_acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let acc = part1(input);
        assert_eq!(5, acc);
    }
}
