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
            let move_x = self.step_east();
            let move_y = self.step_south();
            count += 1;
            if !(move_x || move_y) {
                break;
            }
        }
        count
    }

    fn step_east(&mut self) -> bool {
        let x_max = self.0[0].len();
        let mut moved = false;
        let mut new_bed = self.0.clone();
        for (y, row) in self.0.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if SeaBedSpace::EastFaced == *val
                    && SeaBedSpace::Empty == self.0[y][(x + 1) % x_max]
                {
                    new_bed[y][(x + 1) % x_max] = SeaBedSpace::EastFaced;
                    new_bed[y][x] = SeaBedSpace::Empty;
                    moved = true;
                }
            }
        }
        self.0 = new_bed;
        moved
    }

    fn step_south(&mut self) -> bool {
        let y_max = self.0.len();
        let mut new_bed = self.0.clone();
        let mut moved = false;
        for (y, row) in self.0.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if SeaBedSpace::SouthFaced == *val
                    && SeaBedSpace::Empty == self.0[(y + 1) % y_max][x]
                {
                    new_bed[(y + 1) % y_max][x] = SeaBedSpace::SouthFaced;
                    new_bed[y][x] = SeaBedSpace::Empty;
                    moved = true;
                }
            }
        }
        self.0 = new_bed;
        moved
    }
}

impl core::fmt::Display for SeaBed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for row in &self.0 {
            for s in row {
                match s {
                    SeaBedSpace::EastFaced => write!(f, ">")?,
                    SeaBedSpace::SouthFaced => write!(f, "v")?,
                    _ => write!(f, ".")?,
                }
            }
            writeln!(f)?;
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
