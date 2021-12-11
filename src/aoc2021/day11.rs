static NEIGHBOUR_INDEX: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 11);
    let mut cavern = parse(&input);
    let mut cloned_cavern = cavern.clone();
    let p1 = part1(&mut cavern);
    println!("Part 1: {}", p1);
    let p2 = part2(&mut cloned_cavern);
    println!("Part 2: {}", p2);
}
#[derive(Debug, Clone)]
struct Cavern {
    map: Vec<Vec<u8>>,
}

impl Cavern {
    fn get_neighbour_indexes(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let len = self.map.len() as i8;
        NEIGHBOUR_INDEX
            .iter()
            .map(move |(dx, dy)| (x as i8 + dx, y as i8 + dy))
            .filter(move |(x, y)| *x >= 0 && *y >= 0 && *x < len && *y < len)
            .map(|(x, y)| (x as usize, y as usize))
    }

    fn apply_steps_until_all_flash(&mut self) -> usize {
        let len = self.map.len();
        for i in 1.. {
            if self.apply_step() == len * len {
                return i;
            }
        }
        unreachable!()
    }

    fn apply_steps(&mut self, nb_steps: usize) -> usize {
        (0..nb_steps).fold(0, |acc, _| acc + self.apply_step())
    }

    fn apply_step(&mut self) -> usize {
        // increase everyone by one
        self.map.iter_mut().flatten().for_each(|cell| {
            *cell += 1;
        });
        // use a stack for flash
        let len = self.map.len() as i8;
        let mut stack = vec![];
        for y in 0..len as usize {
            for x in 0..len as usize {
                if self.map[y][x] == 10 {
                    stack.push((x, y));
                }
            }
        }
        // propagate flash
        let mut flash_count = 0;
        while let Some(current) = stack.pop() {
            for (x, y) in self.get_neighbour_indexes(current.0, current.1) {
                self.map[y][x] += 1;
                if self.map[y][x] == 10 {
                    stack.push((x, y));
                }
            }
            flash_count += 1;
        }
        // reset flashed ones to 0
        self.map
            .iter_mut()
            .flatten()
            .filter(|c| **c > 9)
            .for_each(|c| *c = 0);
        flash_count
    }
}

fn parse(input: &str) -> Cavern {
    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    Cavern { map }
}

fn part1(cavern: &mut Cavern) -> usize {
    cavern.apply_steps(100)
}

fn part2(cavern: &mut Cavern) -> usize {
    cavern.apply_steps_until_all_flash()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_1() {
        let input = "11111\n\
                        19991\n\
                        19191\n\
                        19991\n\
                        11111";
        let mut cavern = parse(&input);
        assert_eq!(cavern.apply_step(), 9);
        assert_eq!(cavern.apply_step(), 0);
    }

    #[test]
    fn test_1_2() {
        let input = crate::util::read_file("inputs/2021_11_test.in");
        let mut cavern = parse(&input);
        assert_eq!(cavern.apply_steps(10), 204);
        assert_eq!(cavern.apply_steps(90), 1656 - 204);
    }

    #[test]
    fn test_2_1() {
        let input = crate::util::read_file("inputs/2021_11_test.in");
        let mut cavern = parse(&input);
        assert_eq!(cavern.apply_steps_until_all_flash(), 195);
    }
}
