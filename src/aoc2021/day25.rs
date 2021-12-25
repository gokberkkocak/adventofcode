use itertools::Itertools;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 25);
    let mut s = parse(&input);
    let p1 = s.apply_steps();
    println!("Part 1: {}", p1);
}

fn parse(input: &str) -> SeaBed {
    SeaBed(
        input
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|b| match b {
                        b'>' => SeaBedSpace::EastFaced,
                        b'v' => SeaBedSpace::SouthFaced,
                        _ => SeaBedSpace::Empty,
                    })
                    .collect()
            })
            .collect(),
    )
}

struct SeaBed(Vec<Vec<SeaBedSpace>>);

#[derive(Clone, PartialEq)]
enum SeaBedSpace {
    EastFaced,
    SouthFaced,
    Empty,
}

impl SeaBed {
    fn apply_steps(&mut self) -> usize {
        let mut count = 0;
        loop {
            let move1 = self.step(0, 1);
            let move2 = self.step(1, 0);
            count += 1;
            if !(move1 || move2) {
                break;
            }
        }
        count
    }

    fn step(&mut self, dy: usize, dx: usize) -> bool {
        let (y_max, x_max) = (self.0.len(), self.0[0].len());
        let mut new_bed = vec![vec![SeaBedSpace::Empty; x_max]; y_max];
        let mut moved = false;
        for (y, x) in (0..y_max).cartesian_product(0..x_max) {
            match self.0[y][x] {
                SeaBedSpace::EastFaced if self.0[y][(x + dx) % x_max] == SeaBedSpace::Empty => {
                    new_bed[y][(x + dx) % x_max] = SeaBedSpace::EastFaced;
                    moved = true;
                }
                SeaBedSpace::SouthFaced if self.0[(y + dy) % y_max][x] == SeaBedSpace::Empty => {
                    new_bed[(y + dy) % y_max][x] = SeaBedSpace::SouthFaced;
                    moved = true;
                }
                SeaBedSpace::EastFaced => new_bed[y][x] = SeaBedSpace::EastFaced,
                SeaBedSpace::SouthFaced => new_bed[y][x] = SeaBedSpace::SouthFaced,
                _ => {}
            }
        }
        self.0 = new_bed;
        moved
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_25_test.in");
        let mut s = parse(&input);
        let p1 = s.apply_steps();
        assert_eq!(p1, 58);
    }
}
