use ahash::AHashMap;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 14);
    let mut polymer = parse(&input);
    let p1 = part_core(&mut polymer, 10);
    println!("Part 1: {}", p1);
    let p2 = part_core(&mut polymer, 40 - 10);
    println!("Part 2: {}", p2);
}

fn part_core(polymer: &mut Polymer, steps: usize) -> usize {
    for _ in 0..steps {
        polymer.react();
    }
    polymer.give_answer()
}

fn parse(input: &str) -> Polymer {
    let mut units = AHashMap::new();
    let mut input_parts = input.split("\n\n");
    let unit_array = input_parts.next().unwrap().as_bytes();
    let first = unit_array[0];
    let last = unit_array[unit_array.len() - 1];
    unit_array
        .windows(2)
        .for_each(|window| {
            *units.entry((window[0], window[1])).or_insert(0) += 1;
        });
    let rules = input_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let (lhs, rhs) = parts
                .next()
                .map(|f| (f.as_bytes()[0], f.as_bytes()[1]))
                .unwrap();
            let value = parts.next().map(|f| f.as_bytes()[0]).unwrap();
            ((lhs, rhs), value)
        })
        .collect();
    Polymer {
        units,
        rules,
        first,
        last,
    }
}

struct Polymer {
    units: AHashMap<(u8, u8), usize>,
    rules: AHashMap<(u8, u8), u8>,
    first: u8,
    last: u8,
}

impl Polymer {
    fn react(&mut self) {
        let mut new_units = AHashMap::new();
        for (rule, ins) in self.rules.iter() {
            if let Some(count) = self.units.get(rule) {
                *new_units.entry((rule.0, *ins)).or_insert(0) += count;
                *new_units.entry((*ins, rule.1)).or_insert(0) += count;
            }
        }
        self.units = new_units;
    }

    fn give_answer(&self) -> usize {
        let mut counts = AHashMap::new();
        for ((a, b), count) in self.units.iter() {
            *counts.entry(*a).or_insert(0) += count;
            *counts.entry(*b).or_insert(0) += count;
        }
        // everything except first and last are counted twice
        *counts.entry(self.first).or_insert(0) += 1;
        *counts.entry(self.last).or_insert(0) += 1;
        // everything is counted twice now, so divide by 2
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        (max - min) / 2
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_14_test.in");
        let mut polymer = parse(&input);
        let p1 = part_core(&mut polymer, 10);
        assert_eq!(p1, 1588);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_14_test.in");
        let mut polymer = parse(&input);
        let p1 = part_core(&mut polymer, 40);
        assert_eq!(p1, 2188189693529);
    }
}
