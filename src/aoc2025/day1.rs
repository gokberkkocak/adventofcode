pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2025, 1);
    let p1 = part1(&input);
    println!("part 1: {}", p1);
    let p2 = part2(&input);
    println!("part 2: {}", p2);
}

struct Rotation {
    direction: Direction,
    magnitude: isize,
}

impl Rotation {
    fn apply(&self, current_nb: isize) -> isize {
        match self.direction {
            Direction::Left => (current_nb - self.magnitude).rem_euclid(100),
            Direction::Right => (current_nb + self.magnitude).rem_euclid(100),
        }
    }

    fn apply_with_pass_count(&self, current_nb: isize) -> (isize, isize) {
        let mut res = match self.direction {
            Direction::Left => current_nb - self.magnitude,
            Direction::Right => current_nb + self.magnitude,
        };
        let count = match self.direction {
            Direction::Left => (current_nb - 1).div_euclid(100) - (res - 1).div_euclid(100),
            Direction::Right => res.div_euclid(100), 
        };

        res = res.rem_euclid(100);

        (res, count)
    }
}

enum Direction {
    Left,
    Right,
}

struct Lock {
    rotations: Vec<Rotation>,
}

impl Lock {
    fn from_input(input: &str) -> Self {
        let rotations = input
            .lines()
            .map(|line| {
                let (dir, mag) = line.split_at(1);
                let direction = match dir {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("invalid direction {}", dir),
                };
                let magnitude: isize = mag.parse().expect("invalid magnitude");
                Rotation {
                    direction,
                    magnitude,
                }
            })
            .collect();
        Lock { rotations }
    }

    fn apply_rotations(&self, starting_nb: isize) -> usize {
        self.rotations
            .iter()
            .scan(starting_nb, |state, r| {
                *state = r.apply(*state);
                Some(*state)
            })
            .filter(|x| *x == 0).count()
    }

    fn apply_rotations_with_count(&self, starting_nb: isize) -> usize {
        self.rotations.iter().scan(starting_nb, | state, r | {
            let res = r.apply_with_pass_count(*state);
            *state = res.0;
            Some(res.1)
        }).sum::<isize>() as usize
    }
}

fn part1(input: &str) -> usize {
    let all_rotations = Lock::from_input(&input);
    all_rotations.apply_rotations(50)
}

fn part2(input: &str) -> usize {
    let all_rotations = Lock::from_input(&input);
    all_rotations.apply_rotations_with_count(50)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2025_1_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 3);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2025_1_test.in");
        let p2 = part2(&input);
        assert_eq!(p2, 6);
    }
}
