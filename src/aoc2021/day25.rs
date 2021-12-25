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
                    .map(|b| {
                        let space = match b {
                            b'>' => SeaBedSpace::EastFaced,
                            b'v' => SeaBedSpace::SouthFaced,
                            _ => SeaBedSpace::Empty,
                        };
                        SeaBedState {
                            space,
                            moved: false,
                        }
                    })
                    .collect()
            })
            .collect(),
    )
}

struct SeaBed(Vec<Vec<SeaBedState>>);
#[derive(Clone, PartialEq)]
struct SeaBedState {
    space: SeaBedSpace,
    moved: bool,
}

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
            let move_x = self.step(0, 1);
            let move_y = self.step(1, 0);
            count += 1;
            if !(move_x || move_y) {
                break;
            }
        }
        count
    }

    fn step(&mut self, dy: usize, dx: usize) -> bool {
        let (y_max, x_max) = (self.0.len(), self.0[0].len());
        let mut moved = false;
        for (y, x) in (0..y_max).cartesian_product(0..x_max) {
            match self.0[y][x] {
                SeaBedState {
                    space: SeaBedSpace::EastFaced,
                    moved: false,
                } if self.0[y][(x + dx) % x_max]
                    == SeaBedState {
                        space: SeaBedSpace::Empty,
                        moved: false,
                    } =>
                {
                    self.0[y][(x + dx) % x_max].space = SeaBedSpace::EastFaced;
                    self.0[y][(x + dx) % x_max].moved = true;
                    self.0[y][x].space = SeaBedSpace::Empty;
                    self.0[y][x].moved = true;
                    moved = true;
                }
                SeaBedState {
                    space: SeaBedSpace::SouthFaced,
                    moved: false,
                } if self.0[(y + dy) % y_max][x]
                    == SeaBedState {
                        space: SeaBedSpace::Empty,
                        moved: false,
                    } =>
                {
                    self.0[(y + dy) % y_max][x].space = SeaBedSpace::SouthFaced;
                    self.0[(y + dy) % y_max][x].moved = true;
                    self.0[y][x].space = SeaBedSpace::Empty;
                    self.0[y][x].moved = true;
                    moved = true;
                }
                _ => {}
            }
        }

        self.0
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|s| s.moved = false));
        moved
    }
}

impl core::fmt::Display for SeaBed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for row in &self.0 {
            for s in row {
                match s.space {
                    SeaBedSpace::EastFaced => write!(f, ">")?,
                    SeaBedSpace::SouthFaced => write!(f, "v")?,
                    _ => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
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
