use std::collections::HashMap;

pub fn run() {
    let input = crate::util::get_puzzle_input(2018, 12);
    let garden = parse_input(&input);
    let p1 = part1(garden.clone());
    println!("p1 {}", p1);
    let p2 = part2(garden);
    println!("p2 {}", p2);
}

#[derive(Clone)]
struct Garden {
    state: Vec<bool>,
    rules: HashMap<Vec<bool>, bool>,
    pad: usize,
}

impl Garden {
    fn new(state: Vec<bool>, rules: HashMap<Vec<bool>, bool>) -> Self {
        Self {
            state,
            rules,
            pad: 0,
        }
    }

    fn apply_generation(&mut self) {
        if self.state[0..=5].iter().any(|&i| i) {
            for _ in 0..5 {
                self.state.insert(0, false);
            }
            self.pad += 5;
        }
        if self.state[self.state.len() - 6..self.state.len()]
            .iter()
            .any(|&i| i)
        {
            for _ in 0..5 {
                self.state.push(false);
            }
        }
        let mut clone_state = vec![false; self.state.len()];
        for (i, item) in clone_state.iter_mut().enumerate().take(self.state.len() - 2).skip(2) {
            if self
                .rules
                .get(&self.state[i - 2..=i + 2])
                .filter(|&&b| b)
                .is_some()
            {
                *item = true;
            }
        }
        self.state = clone_state;
        // self._print();
    }

    fn apply_generations(&mut self, n: usize) {
        for _ in 0..n {
            self.apply_generation();
        }
    }

    fn calculate_value(&self) -> isize {
        self.state
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i as isize - self.pad as isize)
            .sum::<isize>()
    }

    fn _print(&self) {
        println!(
            "{}",
            self.state
                .iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

trait MapToBoolVec {
    fn map_to_vec(&self) -> Vec<bool>;
}

impl MapToBoolVec for &str {
    fn map_to_vec(&self) -> Vec<bool> {
        self.chars()
            .map(|c| c == '#')
            .collect()
    }
}

fn parse_input(input: &str) -> Garden {
    let mut rules = HashMap::new();
    let mut lines = input.lines();
    let init_state = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .map_to_vec();
    for line in lines.skip(1) {
        let mut spl = line.split(" => ");
        let rule = spl.next().unwrap().map_to_vec();
        let outcome = spl.next().unwrap().map_to_vec().into_iter().next().unwrap();
        rules.insert(rule, outcome);
    }
    Garden::new(init_state, rules)
}

fn part1(mut garden: Garden) -> isize {
    garden.apply_generations(20);
    garden.calculate_value()
}

fn part2(mut garden: Garden) -> isize {
    // after around 250 generations, it goes in a linear increase pattern.
    garden.apply_generations(250);
    let current = garden.calculate_value();
    garden.apply_generation();
    let increase = garden.calculate_value() - current;
    (50000000000 - 250) * increase + current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2018_12_test.in");
        let garden = parse_input(&input);
        let p1 = part1(garden);
        assert_eq!(p1, 325);
    }
}
