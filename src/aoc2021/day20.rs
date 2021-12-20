use fxhash::FxHashSet;

const ZONE: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 20);
    let mut im = parse(&input);
    let p1 = part_core(&mut im, 2);
    println!("Part 1: {}", p1);
    let p2 = part_core(&mut im, 48);
    println!("Part 1: {}", p2);
}

fn part_core(im: &mut Image, n: usize) -> usize {
    for i in 0..n {
        im.enhance(i % 2 == 1);
    }
    im.count()
}

struct Image {
    image: FxHashSet<(isize, isize)>,
    flip: bool,
    algo: Vec<bool>,
}

fn parse(input: &str) -> Image {
    let mut parts = input.split("\n\n");
    let algo = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    let image = parts
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| (x as isize, y as isize))
        })
        .collect::<FxHashSet<_>>();
    let flip = algo[0];
    Image { image, flip, algo }
}

impl Image {
    fn enhance(&mut self, odd: bool) {
        let mut new_image = FxHashSet::default();
        let min_x = self.image.iter().map(|&(x, _)| x).min().unwrap();
        let max_x = self.image.iter().map(|&(x, _)| x).max().unwrap();
        let min_y = self.image.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = self.image.iter().map(|&(_, y)| y).max().unwrap();

        for y in min_y - 2..max_y + 2 {
            for x in min_x - 2..max_x + 2 {
                let index = ZONE
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, (dx, dy))| {
                        match (
                            self.image.contains(&(x + dx, y + dy)),
                            self.flip,
                            odd,
                            x + dx >= min_x
                                && x + dx <= max_x
                                && y + dy >= min_y
                                && y + dy <= max_y,
                        ) {
                            (true, _, _, _) => 1 << i,
                            (false, true, true, true) => 0,
                            (false, true, true, false) => 1 << i,
                            _ => 0,
                        }
                    })
                    .sum::<usize>();
                if self.algo[index] {
                    new_image.insert((x, y));
                }
            }
        }
        self.image = new_image;
    }

    fn count(&self) -> usize {
        self.image.iter().count()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut lines = vec![];
        let min_x = self.image.iter().map(|&(x, _)| x).min().unwrap();
        let max_x = self.image.iter().map(|&(x, _)| x).max().unwrap();
        let min_y = self.image.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = self.image.iter().map(|&(_, y)| y).max().unwrap();
        for y in min_y..max_y {
            let mut line = String::new();
            for x in min_x..max_x {
                line.push(if self.image.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                });
            }
            lines.push(line);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_20_test.in");
        let mut im = parse(&input);
        let p1 = part_core(&mut im, 2);
        assert_eq!(p1, 35);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_20_test.in");
        let mut im = parse(&input);
        let p1 = part_core(&mut im, 50);
        assert_eq!(p1, 3351);
    }
}
